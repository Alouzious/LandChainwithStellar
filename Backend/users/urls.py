from django.urls import path
from .views import RegisterUser, GoogleAuthView

urlpatterns = [
    path('register/', RegisterUser.as_view(), name='register-user'),   # just 'register/' here, NOT 'api/users/register/'
    path('auth/google/', GoogleAuthView.as_view(), name='google-auth'),
]
