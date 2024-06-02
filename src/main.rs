use std::{fs, io};


fn main() {
    let file = fs::File::open("compress3.zip").unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue
        };

        if (file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());

            fs::create_dir_all(&outpath).unwrap();
        }

        else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );

            let path_parent = outpath.parent();


            if !path_parent.unwrap().exists() {
                fs::create_dir_all(path_parent.unwrap()).unwrap();
            }


            let mut outfile = fs::File::create(&outpath).unwrap();

            io::copy(&mut file,&mut outfile).unwrap();
        }
    }
}
