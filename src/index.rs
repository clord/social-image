use rocket::http::ContentType;

#[get("/", format = "html")]
pub fn index() -> (ContentType, &'static str) {
    (
        ContentType::HTML,
        r##"<!doctype html>
<html>
    <head>
      <meta charset="utf-8">
      <title>social-image</title>
      <meta name="description" content="api to make social images">
      <meta name="viewport" content="width=device-width, initial-scale=1">
      <script src="https://cdn.tailwindcss.com?plugins=typography"></script>
    </head>
    <body class="bg-gray-100">
        <div class="bg-white shadow overflow-hidden sm:rounded-lg max-w-3xl mx-auto my-10">
            <div class="px-4 py-5 sm:px-6 flex flex-row flex-nowrap gap-4 justify-start ">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-10 w-10 place-self-center" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
                </svg>
                <div>
                    <h1 class="text-xl font-bold">social-image usage</h1>
                    <p class="mt-1 max-w-2xl text-sm text-gray-500">Post SVGs to render into other formats</p>
                </div>
            </div>
            <div class="border-t border-gray-200 px-4 py-5 sm:p-0">
                <dl class="sm:divide-y sm:divide-gray-200">
                    <div class="py-4 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                        <dt class="font-medium text-gray-500"><code>GET /</code></dt>
                        <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2 prose"><p>This help content</p></dd>
                    </div>
                    <div class="py-4 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                        <dt class="font-medium text-gray-500"><code>POST /image</code></dt>
                        <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2 prose">
                            <p>Post a SVG and it's resources in a supported format, and response body will contain the PNG.</p>
                            <p>Submit files as <code>multipart/form-data</code>. The <code>svg</code> field contains the main svg to render, 
                               and a series of <code>resources[name]</code> can also be sent for associated files like pngs or fonts.</p>
                            <p>Output size is determined by the SVG's <code>width</code> and <code>height</code> attributes.</p>
                            <h5 class="text-sm font-medium text-gray-500">Headers</h5>
                            <ul>
                                <li>
                                    <code>X-API-KEY</code>: required. Key to access the service.
                                </li>
                            </ul>
                            <h5 class="text-sm font-medium text-gray-500">Example</h5>
                            <pre><code>curl -X POST \
    -H "x-api-key: Your-Private-Key" \
    -F svg=@my_svg.svg \
    -F resources[resource.png]=@resource.png \
    -F resources[greatfont-Regular.otf]=@greatfont-Regular.otf\
    http://localhost:8000/image \
    --output test.png</code></pre>
                        </dd>
                    </div>
                </dl>
            </div>
        </div>
    </body>
</html>
"##,
    )
}
