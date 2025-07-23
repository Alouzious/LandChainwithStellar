from rest_framework import serializers
from django.contrib.auth.models import User

class UserRegistrationSerializer(serializers.ModelSerializer):
    password = serializers.CharField(write_only=True, required=False)

    class Meta:
        model = User
        fields = ['first_name', 'last_name', 'email', 'username', 'password']

    def create(self, validated_data):
        user = User.objects.create_user(
            username=validated_data.get('username') or validated_data['email'],
            email=validated_data['email'],
            first_name=validated_data['first_name'],
            last_name=validated_data['last_name'],
            password=validated_data.get('password', None)
        )
        return user
