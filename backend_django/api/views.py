from rest_framework.decorators import api_view
from rest_framework.response import Response
from rest_framework import status
from .serializer import UserResponseSerializer

@api_view(['GET'])
def get_user(request):
    response = {"name": "pedro", "age": 26}

    return Response(response)

@api_view(['POST'])
def create_user(request):
    serializer = UserResponseSerializer(data=request.data)
    if serializer.is_valid():
        data = serializer.validated_data
        
        return Response(data, status=status.HTTP_201_CREATED)
    
    return Response(serializer.errors, status=status.HTTP_201_CREATED)

@api_view(['GET'])
def get_root(request):
    return Response({ "Hello ayu World" })