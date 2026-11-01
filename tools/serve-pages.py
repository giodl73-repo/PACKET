"""Serve the generated project-prefix demo on loopback for local browser tests."""
from http.server import ThreadingHTTPServer,SimpleHTTPRequestHandler
from functools import partial
from pathlib import Path
root=Path(__file__).resolve().parents[1]
ThreadingHTTPServer(('127.0.0.1',4177),partial(SimpleHTTPRequestHandler,directory=str(root/'dist'))).serve_forever()
