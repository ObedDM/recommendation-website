from django.urls import path
from .views import get_user, create_user, get_root

urlpatterns = [
    path('users/', get_user, name='get_user'),
    path('users/create', create_user, name='create_user'),
    path('', get_root, name="get_root")
]