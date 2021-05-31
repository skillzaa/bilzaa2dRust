use std::time::Duration;
use std::thread;
use bilzaa2d::Bilzaa2d;
fn main(){
    let mut b2d = Bilzaa2d::new();
    
    b2d.play_head.play();
    let a = b2d.add_shape("nima");
    // println!("Shape ======>>>>>>>{:?}",a);    

    a.add_animation( 0,
        10,
        100,
        0,
        "width");
    
    //=========================================    
    //=========================================
    thread::sleep(Duration::from_millis(3000));
    //=========================================    
    //=========================================    
       
        
    println!("Time ======>>>>>>>{:?}",b2d.play_head.time());    
    b2d.update(b2d.play_head.time());
    b2d.draw();    
        
}

