from rest_framework.views import APIView
from rest_framework.response import Response
from rest_framework import status
from .serializers import UserRegistrationSerializer

class RegisterUser(APIView):
    def post(self, request):
        serializer = UserRegistrationSerializer(data=request.data)
        if serializer.is_valid():
            serializer.save()
            return Response({'message': 'User registered successfully'}, status=status.HTTP_201_CREATED)
        return Response(serializer.errors, status=status.HTTP_400_BAD_REQUEST)


from rest_framework.views import APIView
from rest_framework.response import Response
from rest_framework import status
import requests
from django.contrib.auth.models import User
from rest_framework_simplejwt.tokens import RefreshToken
from django.views.decorators.csrf import csrf_exempt
from django.utils.decorators import method_decorator

GOOGLE_TOKEN_INFO_URL = 'https://oauth2.googleapis.com/tokeninfo'

@method_decorator(csrf_exempt, name='dispatch')
class GoogleAuthView(APIView):
    def post(self, request):
        id_token = request.data.get('id_token')

        if not id_token:
            return Response({"error": "ID token is required"}, status=status.HTTP_400_BAD_REQUEST)

        # ✅ Verify the token with Google
        token_response = requests.get(GOOGLE_TOKEN_INFO_URL, params={'id_token': id_token})
        if token_response.status_code != 200:
            return Response({"error": "Invalid Google token"}, status=status.HTTP_400_BAD_REQUEST)

        token_info = token_response.json()
        email = token_info.get('email')
        first_name = token_info.get('given_name')
        last_name = token_info.get('family_name')

        if not email:
            return Response({"error": "Google token did not return an email"}, status=status.HTTP_400_BAD_REQUEST)

        # ✅ Check if user exists, otherwise create one
        user, created = User.objects.get_or_create(email=email, defaults={
            'username': email,
            'first_name': first_name or '',
            'last_name': last_name or '',
        })

        # ✅ Create JWT token
        refresh = RefreshToken.for_user(user)

        return Response({
            'token': str(refresh.access_token),
            'refresh': str(refresh),
            'user': {
                'email': user.email,
                'first_name': user.first_name,
                'last_name': user.last_name,
            }
        })
