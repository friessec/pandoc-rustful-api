# Current Version
v4.10.3

# Installation
- Download latest swagger-ui from https://github.com/swagger-api/swagger-ui
- Extract files from dist to this folder
- Change `url` in `swagger-initializer.js` file to `./swagger.json`
- For development change `validatorUrl` to null
```js
window.ui = SwaggerUIBundle({
    url: './swagger.json',
    dom_id: '#swagger-ui',
    validatorUrl: null,
    deepLinking: true,
    presets: [
        SwaggerUIBundle.presets.apis,
        SwaggerUIStandalonePreset
    ],
    plugins: [
        SwaggerUIBundle.plugins.DownloadUrl
    ],
    layout: "StandaloneLayout"
});
```

