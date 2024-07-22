# Conway's Game of Life
[Gif con el resultado del Juego](https://github.com/user-attachments/files/16326801/Tarea.de.Resumen.1.Costos.de.un.Proyecto.de.SW.1.pdf)
![conway](https://github.com/user-attachments/assets/33a605bc-9afe-465d-bd8e-45352293fe2f)

## Descripción

Este proyecto es una implementación del Juego de la Vida de Conway en Rust. El juego simula autómatas celulares en una cuadrícula bidimensional. Los organismos (células) pueden estar vivas o muertas, y el estado de cada célula en la siguiente iteración se determina por el número de células vecinas vivas.

La aplicación utiliza `minifb` para manejar la ventana gráfica y `nalgebra_glm` para operaciones matemáticas con vectores.

## Requisitos

- Rust (última versión estable)
- Cargo (gestor de paquetes de Rust)
- `minifb` y `nalgebra_glm` (dependencias del proyecto)

## Contenido del Proyecto

- **Cargo.toml**: Archivo de configuración de Cargo con las dependencias del proyecto.

- **Cargo.lock**: Archivo de bloqueo de dependencias.

- **README.md**: Este archivo.

- **src/**: Directorio que contiene el código fuente del proyecto.
  
  - **bmp.rs**: 
    - **Descripción**: Módulo para manejar archivos BMP. Proporciona funciones para leer y escribir imágenes en formato BMP.

  - **color.rs**: 
    - **Descripción**: Módulo que define los colores utilizados en el framebuffer. Incluye estructuras y constantes para representar y manipular colores en formato hexadecimal.

  - **framebuffer.rs**: 
    - **Descripción**: Módulo para el manejo del framebuffer y la representación gráfica. Define la estructura `Framebuffer`, que gestiona el buffer de píxeles y proporciona métodos para dibujar puntos y limpiar el framebuffer.

  - **line_impl.rs**: 
    - **Descripción**: Implementación de algoritmos para la manipulación de líneas. Contiene funciones para dibujar líneas utilizando algoritmos como Bresenham, útiles para la visualización en el framebuffer.

  - **main.rs**: 
    - **Descripción**: Punto de entrada principal del proyecto que configura y ejecuta el juego. Inicializa la ventana, genera patrones aleatorios, actualiza la cuadrícula y maneja la lógica del Juego de la Vida.

  - **mod.rs**: 
    - **Descripción**: Archivo de módulo principal para la estructura del proyecto. Declara y organiza los módulos que se utilizan en el proyecto, facilitando la importación y el uso de los diferentes archivos fuente.

  - **vertex.rs**: 
    - **Descripción**: Módulo para la definición y manejo de vértices. Incluye estructuras y funciones relacionadas con la manipulación de vértices en gráficos, esenciales para las operaciones de dibujo.


