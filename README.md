# 🦀 RsPDF

**RustyPDF** es una librería escrita en Rust para generar documentos **PDF** de forma programática. Este proyecto es parte de un proceso de aprendizaje, con el objetivo de entender a fondo cómo funciona internamente el formato PDF, y al mismo tiempo ofrecer una API sencilla y poderosa para crear documentos desde cero.

> 📄 Crea PDFs con texto, líneas y múltiples tamaños de página.  
> 🧠 Ideal para aprender sobre estructuras PDF y programación en Rust.

---

## ✨ Características

- ✅ Crear **páginas** en diferentes **tamaños** y **orientaciones** (`Portrait` / `Landscape`).
- ✅ Añadir **textos** personalizados con posición, tamaño y color.
- ✅ Dibujar **líneas** con coordenadas, grosor y color.

---

## 🚧 Pendiente por implementar

- 🖼️ Soporte para **imágenes** (JPG, PNG, etc).
- 🔤 Carga de **fuentes personalizadas**.
- 🆎 Soporte para **estilos tipográficos** (`Font Face`).

---

## 🚀 Ejemplo de uso

```rust
let mut pdf = RsPdf::new("My first PDF");

// Página A4 en orientación vertical
let mut page = Page::new(OrientationType::Portrait, PageFormat::A4.get_format());
page.add_content(Text::new("Hello World!", "F1", 16, Point(100.0, page.size().height_value() - 200.0), RGB(87, 150, 100)).into());
page.add_content(Text::new("Other text!", "F1", 12, Point(0.0, page.size().height_value() - 20.0), RGB(74, 7, 7)).into());
page.add_content(Line::new(Point(20.0, 200.0), Point(page.size().width_value() - 20.0, 200.0), RGB(0, 0, 0), 1.0).into());

// Página Tabloid
let mut page_tabloid = Page::new(OrientationType::Portrait, PageFormat::Tabloid.get_format());
page_tabloid.add_content(Text::new("Hello", "F1", 42, Point(100.0, 700.0), RGB(87, 150, 100)).into());
page_tabloid.add_content(Line::new(Point(20.0, 200.0), Point(page.size().width_value() - 20.0, 700.0), RGB(0, 0, 0), 1.0).into());

pdf.add_page(page);
pdf.add_page(page_tabloid);

println!("{:?}", pdf.build());
```

## 📦 Instalación

Este proyecto aún no se publica en crates.io. Si quieres probarlo, puedes clonarlo localmente::

```bash
git clone https://github.com/tu_usuario/rustypdf.git
cd rustypdf
cargo run
```