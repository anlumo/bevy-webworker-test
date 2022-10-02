# -*- coding: utf-8 -*-
import http.server
from http.server import HTTPServer, BaseHTTPRequestHandler
import socketserver

PORT = 8000

class Handler(http.server.SimpleHTTPRequestHandler):
    extensions_map = {
        "": "application/octet-stream",
        ".css": "text/css",
        ".html": "text/html",
        ".jpg": "image/jpg",
        ".js": "application/javascript",
        ".json": "application/json",
        ".manifest": "text/cache-manifest",
        ".pdf": "application/pdf",
        ".png": "image/png",
        ".svg": "image/svg+xml",
        ".wasm": "application/wasm",
        ".xml": "application/xml",
    }
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory="wasm", **kwargs)

httpd = socketserver.TCPServer(("localhost", PORT), Handler)

print("serving at port", PORT)
try:
    httpd.serve_forever()
except KeyboardInterrupt:
    exit()