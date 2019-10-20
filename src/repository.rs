use std::io;
use std::io::prelude::*;
use std::fs::File;

use crate::util as ul;

pub struct DataProvider {
    images_source: File,
    labels_source: File,
    pub total: u32,
    count: u32,
}

impl DataProvider {
    pub fn new(images_src: &str, labels_src: &str) -> DataProvider {
        let mut provider = DataProvider {
            images_source: File::open(images_src).unwrap(),
            labels_source: File::open(labels_src).unwrap(),
            total: 0u32,
            count: 0u32,
        };

        let _image_magic = ul::read_next_head(&mut provider.images_source);
        let image_total = ul::read_next_head(&mut provider.images_source);
        let _image_row = ul::read_next_head(&mut provider.images_source);
        let _image_column = ul::read_next_head(&mut provider.images_source);

        let _label_magic = ul::read_next_head(&mut provider.labels_source);
        let label_total = ul::read_next_head(&mut provider.labels_source);

        if image_total == label_total {
            provider.total = image_total;
        } else {
            panic!("image and label's file is not match!");
        }

        provider
    }

    /// returns optional tuple of image and label
    pub fn next(&mut self) -> Option<([u8; ul::IMAGE_SIZE], u8)> {
        self.count += 1;

        if self.count > self.total {
            None
        } else {
            let buffer_image: [u8; ul::IMAGE_SIZE] = ul::read_next_image(&mut self.images_source);
            let buffer_label: u8 = ul::read_next_label(&mut self.labels_source);
            Some((buffer_image, buffer_label))
        }
    }
}