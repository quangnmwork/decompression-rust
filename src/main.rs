use std::{fs, io};


fn main() {
    let file = fs::File::open("compress3.zip").unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    /**
        There can be multiple files.
     */
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        /**
            Ensure zip file exist
        */
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue
        };

        /**
            If input is a folder path.
         */
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

            /** Create parent directory */
            if !path_parent.unwrap().exists() {
                fs::create_dir_all(path_parent.unwrap()).unwrap();
            }

            /**
                Create out file
             */
            let mut outfile = fs::File::create(&outpath).unwrap();

            /**
                Copy content file in zip folder to outfile
             */
            io::copy(&mut file,&mut outfile).unwrap();
        }
    }
}
