from rest_framework import serializers

class UserResponseSerializer(serializers.Serializer):
    user_id = serializers.UUIDField()
    username = serializers.CharField(max_length=18)