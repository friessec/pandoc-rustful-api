
- Download latest swagger-ui from https://github.com/swagger-api/swagger-ui
- Extract files from dist to this folder
- Change `url` in `index.html` file to `./swagger.json`
```js
const ui = SwaggerUIBundle({
        url: "./swagger.json",
        dom_id: '#swagger-ui',
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