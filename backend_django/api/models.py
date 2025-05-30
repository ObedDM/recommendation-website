from django.db import models

# Create your models here.
class UserResponses():

    def __init__(self, user_id, username):
        self.user_id = user_id
        self.username = username