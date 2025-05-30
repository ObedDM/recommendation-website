from rest_framework import serializers
from models import UserResponses

class UserSerializer(serializers.ModelSerialzer):
    class Meta:
        model = UserResponses
        fields = '__all__'