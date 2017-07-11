extern crate serde;
extern crate serde_pickle;
use std::path::Path;
use std::fs::File;

struct Key_files {
    train_img: String,
    test_img: String,
    train_label: String,
    test_label: String,
}

trait MNIST {
  fn load_mnist(&self, normalize: bool, flatten: bool, one_hot_lobel: bool) -> Key_file ;
}

impl MNIST for Key_files {
    //TODO: mnist
  fn load_mnist(&self, normalize: bool, flatten: bool, one_hot_lobel: bool) -> Key_file {

    // try!(Path::new("/mnist.pkl").exsits());
    let file_path = Path::new("mnist.pkl").exsits().expect("pkl_file not exist. ");
    let mut dataset = serde_pickle::value_from_reader(file_path);
    if nomalize  {
      let dataset = (0..1).map ( |dataset| 
        dataset[key] = dataset[key] as f64;
	    dataset[key]/255.0
      )
    }

   // if one_hot_label {
   //   dataset[2] = change_hot_label(dataset[2]);
   //   dataset[3] = change_hot_label(dataset[3]);
   // }

    if !faltten  {
      for key in 0..1 {
        //dataset[key] = dataset[key].reshape(-1,1,28,28);
      }
    }
  }
}
