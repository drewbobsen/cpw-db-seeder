use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

// Note: #[tokio::main] is gone. We are running purely synchronous file I/O now.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Ensure the output directory exists
    let output_dir = "./data";
    fs::create_dir_all(output_dir)?;
    
    let output_path = format!("{}/seed.sql", output_dir);
    let file = File::create(&output_path)?;
    let mut writer = BufWriter::new(file);

    println!("Generating SQL seed file at: {}", output_path);

    // 2. Write the schema to the top of the file
    writeln!(writer, "-- CompareTheWord Database Seed File\n")?;
    
    writeln!(writer, "CREATE TABLE IF NOT EXISTS translations (")?;
    writeln!(writer, "    code VARCHAR(50) PRIMARY KEY,")?;
    writeln!(writer, "    name VARCHAR(255) NOT NULL")?;
    writeln!(writer, ");\n")?;

    writeln!(writer, "CREATE TABLE IF NOT EXISTS verses (")?;
    writeln!(writer, "    id SERIAL PRIMARY KEY,")?;
    writeln!(writer, "    translation_code VARCHAR(50) REFERENCES translations(code),")?;
    writeln!(writer, "    book VARCHAR(50) NOT NULL,")?;
    writeln!(writer, "    chapter INT NOT NULL,")?;
    writeln!(writer, "    verse INT NOT NULL,")?;
    writeln!(writer, "    text TEXT NOT NULL,")?;
    writeln!(writer, "    UNIQUE(translation_code, book, chapter, verse)")?;
    writeln!(writer, ");\n")?;

    let dir_path = "./text/en"; 
    println!("Scanning directory: {}", dir_path);

    // 3. Iterate through every file
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("xml") {
            let translation_code = path.file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            println!("Processing translation: {} from file: {:?}", translation_code, path);
            
            // Pass the writer down so the parser can append to the file
            parse_and_write(&path, &translation_code, &mut writer)?;
        }
    }

    // 4. Force the writer to flush any remaining data to disk
    writer.flush()?;
    println!("All translations parsed and saved to {}!", output_path);
    
    Ok(())
}

// 5. The modified parser function
fn parse_and_write(
    file_path: &Path, 
    translation_code: &str, 
    writer: &mut BufWriter<File>
) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = Reader::from_file(file_path)?;
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    
    let mut in_title = false;
    let mut translation_name = String::new();
    let mut title_saved = false;

    let mut current_book = String::new();
    let mut current_chapter: i32 = 0;
    let mut current_verse: i32 = 0;
    let mut in_verse = false;
    writeln!(writer, "BEGIN;")?;
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let name_str = String::from_utf8_lossy(e.name().into_inner());
                
                if name_str == "title" && translation_name.is_empty() {
                    in_title = true;
                }
                
                if name_str == "verse" {
                    for attr in e.attributes().flatten() {
                        if attr.key.as_ref() == b"osisID" {
                            let osis_id = String::from_utf8_lossy(&attr.value);
                            let parts: Vec<&str> = osis_id.split('.').collect();
                            
                            if parts.len() == 3 {
                                current_book = parts[0].to_string();
                                current_chapter = parts[1].parse().unwrap_or(0);
                                current_verse = parts[2].parse().unwrap_or(0);
                            }
                        }
                    }
                    in_verse = true;
                }
            }
            Ok(Event::Text(e)) => {
                if in_title {
                    let raw = String::from_utf8_lossy(e.as_ref());
                    translation_name = quick_xml::escape::unescape(&raw).unwrap_or_else(|_| raw.clone()).into_owned();
                }
                
                if in_verse {
                    let raw = String::from_utf8_lossy(e.as_ref());
                    let text = quick_xml::escape::unescape(&raw).unwrap_or_else(|_| raw.clone()).into_owned();
                    
                    

                    if !title_saved && !translation_name.is_empty() {
                        // Escape single quotes for SQL insertion
                        let safe_name = translation_name.replace("'", "''");
                        
                        writeln!(
                            writer,
                            "INSERT INTO translations (code, name) VALUES ('{}', '{}') ON CONFLICT (code) DO NOTHING;",
                            translation_code, safe_name
                        )?;
                        
                        println!("Registered: {} ({})", translation_name, translation_code);
                        title_saved = true;
                    }

                    

                    // Escape single quotes in the verse text
                    let safe_text = text.replace("'", "''");

                    // Write the upsert statement
                    writeln!(
                        writer,
                        "INSERT INTO verses (translation_code, book, chapter, verse, text) VALUES ('{}', '{}', {}, {}, '{}') ON CONFLICT (translation_code, book, chapter, verse) DO UPDATE SET text = EXCLUDED.text;",
                        translation_code, current_book, current_chapter, current_verse, safe_text
                    )?;

                }
            }
            Ok(Event::End(ref e)) => {
                let name_str = String::from_utf8_lossy(e.name().into_inner());
                
                if name_str == "title" {
                    in_title = false;
                }
                if name_str == "verse" {
                    in_verse = false;
                }
            }
            Ok(Event::Eof) => break, 
            Err(e) => {
                eprintln!("Error parsing file {:?} at position {}: {:?}", file_path, reader.buffer_position(), e);
                break;
            }
            _ => (),
        }
        
        buf.clear();
    }
    writeln!(writer, "COMMIT;")?;
    Ok(())
}