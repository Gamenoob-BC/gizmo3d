from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="gizmo_py",
    version="0.1.0",
    rust_extensions=[RustExtension("gizmo_engine.gizmo3d", "Cargo.toml", binding=Binding(1))],
    packages=["gizmo_py"],
    zip_safe=False,
)
