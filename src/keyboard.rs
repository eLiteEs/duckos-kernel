use x86_64::instructions::port::Port;
use crate::framebuffer::{WRITER, INPUT_PROMPT};
use spin::Mutex;

// Buffer para la línea actual
static INPUT_BUFFER: Mutex<[u8; 256]> = Mutex::new([0; 256]);
static INPUT_LEN: Mutex<usize> = Mutex::new(0);

// Mapa de scancodes a ASCII (scancode set 1)
fn scancode_to_ascii(sc: u8, shift: bool) -> Option<char> {
    match sc {
        // Letras minúsculas
        0x10 => Some(if shift { 'Q' } else { 'q' }),
        0x11 => Some(if shift { 'W' } else { 'w' }),
        0x12 => Some(if shift { 'E' } else { 'e' }),
        0x13 => Some(if shift { 'R' } else { 'r' }),
        0x14 => Some(if shift { 'T' } else { 't' }),
        0x15 => Some(if shift { 'Y' } else { 'y' }),
        0x16 => Some(if shift { 'U' } else { 'u' }),
        0x17 => Some(if shift { 'I' } else { 'i' }),
        0x18 => Some(if shift { 'O' } else { 'o' }),
        0x19 => Some(if shift { 'P' } else { 'p' }),
        0x1E => Some(if shift { 'A' } else { 'a' }),
        0x1F => Some(if shift { 'S' } else { 's' }),
        0x20 => Some(if shift { 'D' } else { 'd' }),
        0x21 => Some(if shift { 'F' } else { 'f' }),
        0x22 => Some(if shift { 'G' } else { 'g' }),
        0x23 => Some(if shift { 'H' } else { 'h' }),
        0x24 => Some(if shift { 'J' } else { 'j' }),
        0x25 => Some(if shift { 'K' } else { 'k' }),
        0x26 => Some(if shift { 'L' } else { 'l' }),
        0x2C => Some(if shift { 'Z' } else { 'z' }),
        0x2D => Some(if shift { 'X' } else { 'x' }),
        0x2E => Some(if shift { 'C' } else { 'c' }),
        0x2F => Some(if shift { 'V' } else { 'v' }),
        0x30 => Some(if shift { 'B' } else { 'b' }),
        0x31 => Some(if shift { 'N' } else { 'n' }),
        0x32 => Some(if shift { 'M' } else { 'm' }),

        // Números
        0x02 => Some(if shift { '!' } else { '1' }),
        0x03 => Some(if shift { '@' } else { '2' }),
        0x04 => Some(if shift { '#' } else { '3' }),
        0x05 => Some(if shift { '$' } else { '4' }),
        0x06 => Some(if shift { '%' } else { '5' }),
        0x07 => Some(if shift { '^' } else { '6' }),
        0x08 => Some(if shift { '&' } else { '7' }),
        0x09 => Some(if shift { '*' } else { '8' }),
        0x0A => Some(if shift { '(' } else { '9' }),
        0x0B => Some(if shift { ')' } else { '0' }),

        // Símbolos
        0x0C => Some(if shift { '_' } else { '-' }),
        0x0D => Some(if shift { '+' } else { '=' }),
        0x1A => Some(if shift { '{' } else { '[' }),
        0x1B => Some(if shift { '}' } else { ']' }),
        0x27 => Some(if shift { ':' } else { ';' }),
        0x28 => Some(if shift { '"' } else { '\'' }),
        0x29 => Some(if shift { '~' } else { '`' }),
        0x2B => Some(if shift { '|' } else { '\\' }),
        0x33 => Some(if shift { '<' } else { ',' }),
        0x34 => Some(if shift { '>' } else { '.' }),
        0x35 => Some(if shift { '?' } else { '/' }),

        // Espacio
        0x39 => Some(' '),

        // Enter
        0x1C => Some('\n'),

        // Backspace
        0x0E => Some('\x08'),

        // Resto
        _ => None,
    }
}

// Comprobar si hay tecla disponible (como tu keyboard_key_available())
pub fn key_available() -> bool {
    unsafe {
        let mut status_port: Port<u8> = Port::new(0x64);  // AÑADIDO mut
        (status_port.read() & 1) != 0
    }
}

// Leer scancode (como tu keyboard_read_scancode())
pub fn read_scancode() -> u8 {
    unsafe {
        let mut data_port: Port<u8> = Port::new(0x60);    // AÑADIDO mut
        data_port.read()
    }
}

// Inicialización mínima del teclado
pub fn init() {
    // Vaciar buffer inicial si hay algo
    unsafe {
        let mut status_port: Port<u8> = Port::new(0x64);  // AÑADIDO mut
        let mut data_port: Port<u8> = Port::new(0x60);    // AÑADIDO mut

        while (status_port.read() & 1) != 0 {
            data_port.read();
        }
    }
}

// Procesar teclas (llamar desde el bucle principal)
pub fn poll_keyboard() {
    if !key_available() {
        return;
    }

    let sc = read_scancode();

    // Ignorar teclas liberadas (bit 7 = 1)
    if sc & 0x80 != 0 {
        return;
    }

    // Convertir scancode a ASCII (sin shift por ahora)
    if let Some(c) = scancode_to_ascii(sc, false) {
        handle_character(c);
    }
}

fn handle_character(c: char) {
    let mut writer = WRITER.lock();

    match c {
        '\n' => {
            if let Some(fb) = writer.as_mut() {
                unsafe {
                    fb.print("\n");

                    // Mostrar lo que se escribió
                    let buffer = INPUT_BUFFER.lock();
                    let len = *INPUT_LEN.lock();
                    fb.print("Has escrito: ");
                    if len > 0 {
                        let mut temp = [0u8; 256];
                        for i in 0..len {
                            temp[i] = buffer[i];
                        }
                        let s = core::str::from_utf8(&temp[..len]).unwrap_or("");
                        fb.print(s);
                    }
                    fb.print("\n");

                    // Limpiar buffer
                    *INPUT_LEN.lock() = 0;
                    fb.print(INPUT_PROMPT);
                }
            }
        }
        '\x08' => { // Backspace
            if let Some(fb) = writer.as_mut() {
                unsafe {
                    let mut len = INPUT_LEN.lock();
                    if *len > 0 {
                        *len -= 1;

                        // Volver a mostrar la línea
                        fb.print("\r");
                        fb.print(INPUT_PROMPT);
                        let buffer = INPUT_BUFFER.lock();
                        let mut temp = [0u8; 256];
                        for i in 0..*len {
                            temp[i] = buffer[i];
                        }
                        let s = core::str::from_utf8(&temp[..*len]).unwrap_or("");
                        fb.print(s);
                        fb.print(" ");
                    }
                }
            }
        }
        c if c.is_ascii_graphic() || c == ' ' => {
            if let Some(fb) = writer.as_mut() {
                unsafe {
                    let mut len = INPUT_LEN.lock();
                    let mut buffer = INPUT_BUFFER.lock();
                    if *len < 255 {
                        buffer[*len] = c as u8;
                        *len += 1;

                        let mut temp = [0u8; 1];
                        temp[0] = c as u8;
                        let s = core::str::from_utf8(&temp).unwrap_or("");
                        fb.print(s);
                    }
                }
            }
        }
        _ => {}
    }
}
