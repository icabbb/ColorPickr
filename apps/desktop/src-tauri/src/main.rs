#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
  )]
  
  use tauri::{Manager};
  use log::{info, error};
  use std::thread;
  use rdev::{listen, Event, EventType, Button};
  use winapi::shared::windef::{POINT, RECT};
  use winapi::um::winuser::{GetCursorPos, GetDC, ReleaseDC, GetSystemMetrics, GetWindowRect, SM_CXSCREEN, SM_CYSCREEN};
  use winapi::um::wingdi::{BitBlt, GetDIBits, CreateCompatibleDC, CreateCompatibleBitmap, SRCCOPY, SelectObject, DeleteObject, DeleteDC};
  use winapi::um::wingdi::{BITMAPINFOHEADER, BI_RGB};
  use std::ptr::null_mut;
  
  fn capture_color_at_cursor(hwnd: isize) -> Result<String, String> {
      unsafe {
          info!("Iniciando captura de color");
          
          // Obtener la posición del cursor
          let mut point: POINT = POINT { x: 0, y: 0 };
          if GetCursorPos(&mut point) == 0 {
              error!("No se pudo obtener la posición del cursor");
              return Err("No se pudo obtener la posición del cursor".into());
          }
  
          // Obtener las coordenadas de la ventana de la aplicación
          let mut rect: RECT = std::mem::zeroed();
          if GetWindowRect(hwnd as *mut _, &mut rect) == 0 {
              error!("No se pudo obtener el rectángulo de la ventana");
              return Err("No se pudo obtener el rectángulo de la ventana".into());
          }
  
          // Verificar si el cursor está dentro de la ventana de la aplicación
          if point.x >= rect.left && point.x <= rect.right && point.y >= rect.top && point.y <= rect.bottom {
              info!("El cursor está dentro de la ventana de la aplicación, omitiendo captura");
              return Err("El cursor está dentro de la ventana de la aplicación".into());
          }
  
          // Obtener el tamaño de la pantalla
          let width = GetSystemMetrics(SM_CXSCREEN);
          let height = GetSystemMetrics(SM_CYSCREEN);
  
          // Crear un contexto de dispositivo (DC) compatible y un bitmap
          let hdc_screen = GetDC(null_mut());
          let hdc_mem = CreateCompatibleDC(hdc_screen);
          let hbm_screen = CreateCompatibleBitmap(hdc_screen, width, height);
          SelectObject(hdc_mem, hbm_screen as *mut _);
  
          // Copiar la pantalla al DC de memoria
          BitBlt(hdc_mem, 0, 0, width, height, hdc_screen, 0, 0, SRCCOPY);
  
          // Preparar para extraer los bits del bitmap
          let mut bmi = BITMAPINFOHEADER {
              biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
              biWidth: width,
              biHeight: -height, // negativo para evitar imagen volteada
              biPlanes: 1,
              biBitCount: 24, // 24-bit color
              biCompression: BI_RGB,
              biSizeImage: 0,
              biXPelsPerMeter: 0,
              biYPelsPerMeter: 0,
              biClrUsed: 0,
              biClrImportant: 0,
          };
  
          let mut buf = vec![0u8; (width * height * 3) as usize];
          if GetDIBits(hdc_mem, hbm_screen, 0, height as u32, buf.as_mut_ptr() as *mut _, &mut bmi as *mut _ as *mut _, 0) == 0 {
              error!("No se pudo obtener los bits del bitmap");
              return Err("No se pudo obtener los bits del bitmap".into());
          }
  
          // Liberar recursos
          ReleaseDC(null_mut(), hdc_screen);
          DeleteObject(hbm_screen as _);
          DeleteDC(hdc_mem);
  
          // Calcular el índice del píxel
          let x = point.x as usize;
          let y = point.y as usize;
          let index = ((y * width as usize + x) * 3) as usize;
  
          // Obtener los valores RGB
          let b = buf[index];
          let g = buf[index + 1];
          let r = buf[index + 2];
  
          let color_hex = format!("#{:02X}{:02X}{:02X}", r, g, b);
          info!("Color capturado: {}", color_hex);
  
          Ok(color_hex)
      }
  }
  
  fn main() {
      env_logger::init(); // Inicia el logger
  
      tauri::Builder::default()
          .setup(|app| {
              let window = app.get_window("main").unwrap();
              let hwnd = window.hwnd().unwrap().0; // Obtener el HWND directamente desde Tauri
  
              thread::spawn(move || {
                  listen(move |event: Event| {
                      if let EventType::ButtonPress(Button::Left) = event.event_type {
                          match capture_color_at_cursor(hwnd) {
                              Ok(color) => {
                                  info!("Emitiendo color al frontend: {}", color);
                                  window.emit("color-update", color).unwrap();
                              },
                              Err(err) => {
                                  error!("Error durante la captura: {}", err);
                              },
                          }
                      }
                  })
                  .expect("Error al capturar eventos globales");
              });
  
              Ok(())
          })
          .run(tauri::generate_context!())
          .expect("error while running tauri application");
  }
  