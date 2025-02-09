use std::{fs::File, io::Write};

pub fn create_entrypoint(project_name: &str) {
    let html = format!(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>{}</title>
</head>
<body>
    <script>
      document.addEventListener('DOMContentLoaded', () => {{
          if ('serviceWorker' in navigator) {{
              navigator.serviceWorker.register('./sw.js');
          }}
      }});
    </script>
    <script src="./js/global.js"></script>
    <script type="module">
      import init from './pkg/{}.js';

      async function main() {{
          const wasm = await init();
      }}

      main();
    </script>
</body>
</html>
"#,
        project_name, project_name
    );

    let mut file = File::create("index.html").expect("Failed to create index.html");
    file.write_all(html.as_bytes())
        .expect("Failed to create index.html");
    file.flush().expect("Failed to create index.html");
}
