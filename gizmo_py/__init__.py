# gizmo_py/__init__.py

# Import your Rust-generated Python module
from . import gizmo_py

# Define a user-friendly API for using your module
def create_game_object(name):
    return gizmo_py.create_game_object(name)

def move_game_object(name, x, y, z):
    return gizmo_py.move_game_object(name, x, y, z)
