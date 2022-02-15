fn main(){
    let m1 : f32 = 2.;
    let m2 : f32 = 4.;
    const array : [f32;9] = [2.0,3.0,4.0,10.0,11.0,12.0,20.0,25.0,30.0];
    let mut c1 : Vec<f32> = Vec::new();
    let mut c2 : Vec<f32> = Vec::new();
    
    for i in 0..array.len(){
      if (array[i]-m1).powf(2.0) > (array[i]-m2).powf(2.0){
            
            c2.push(array[i]);
      }
      else{
     
            c1.push(array[i]);
      }
    }

    let mut m1 : f32 = 0.;
    for i in 0..c1.len(){
       m1 += c1[i];
    }
    m1 /= c1.len() as f32;
    
    let mut m2 : f32 = 0.;
    for i in 0..c2.len(){
       m2 += c2[i];
    }
    m2 /= c2.len() as f32;

      println!("{:?}", c1);
      println!("{:?}", c2);
      println!("{}",m1);
      println!("{}",m2);
   

      for i in 0..c1.len(){
         if (c1[i]-m1).powf(2.0) > (c1[i]-m2).powf(2.0){
            
            c2.push(c1[i]);
            let b : f32 = c1[i];
            c1.retain(|x| *x != b);  
            break;          
            
         }
      }
      
      for i in 0..c2.len(){
         if (c2[i]-m2).powf(2.0) > (c2[i]-m1).powf(2.0){
     
            c1.push(c2[i]);
            let b : f32 = c2[i];
            c2.retain(|x| *x != b);
            break;
         }
      }
      
      let mut v1 : Vec<f32> = Vec::new();
      let mut v2 : Vec<f32> = Vec::new();
      v1.push(m1);
      v2.push(m2);
      
    let mut a = 0;
   loop {
    let mut m1 : f32 = 0.;
    for i in 0..c1.len(){
       m1 += c1[i];
    }
    m1 /= c1.len() as f32;
    
    let mut m2 : f32 = 0.;
    for i in 0..c2.len(){
       m2 += c2[i];
    }
    m2 /= c2.len() as f32;

      println!("{:?}", c1);
      println!("{:?}", c2);
      println!("{}",m1);
      println!("{}",m2);
   

      for i in 0..c1.len(){
         if (c1[i]-m1).powf(2.0) > (c1[i]-m2).powf(2.0){
            
            c2.push(c1[i]);
            let b : f32 = c1[i];
            c1.retain(|x| *x != b);  
            break;          
            
         }
      }
      
      for i in 0..c2.len(){
         if (c2[i]-m2).powf(2.0) > (c2[i]-m1).powf(2.0){
     
            c1.push(c2[i]);
            let b : f32 = c2[i];
            c2.retain(|x| *x != b);
            break;
         }
      }

      v1.push(m1);
      v2.push(m2);
      
      a += 1;
      if v1[a] == v1[a-1] && v2[a] == v2[a-1]{
        break;
      }
    }
}

fn main(){
    let m1 : f32 = 2.;
    let m2 : f32 = 4.;
    const array : [f32;9] = [2.0,3.0,4.0,10.0,11.0,12.0,20.0,25.0,30.0];
    let mut c1 : Vec<f32> = Vec::new();
    let mut c2 : Vec<f32> = Vec::new();
    
    let mut kernel : [[f32; array.len()]; array.len()] = [[0.0;array.len()];array.len()];

    for i in 0..array.len(){
      for j in 0..array.len(){
      kernel[i][j] = array[i]*array[j];
      }
    }
    println!("kernel matrix");
    println!("{:?}", kernel);
    
    
    for i in 0..array.len(){
      if (array[i]-m1).powf(2.0) > (array[i]-m2).powf(2.0){
            
            c2.push(array[i]);
      }
      else{
     
            c1.push(array[i]);
      }
    }


      println!("{:?}", c1);
      println!("{:?}", c2);
    
    
    
    for i in 0..c1.len(){
      
         let mut g: f32 = 0.;
         for j in 0..c1.len(){
            for k in 0..c1.len(){
               g += kernel[j][k];
            }
         }
         g /= (c1.len() as f32).powf(2.0);
         let mut s: f32 = 0.;
         for j in 0..c1.len(){
            s += kernel[j][i];
         }
         s /= c1.len() as f32;
         
         let mut w: f32 = 0.;
         for j in c1.len()..c1.len()+c2.len(){
            for k in c1.len()..c1.len()+c2.len(){
               w += kernel[j][k];
            }
         }
         w /= (c2.len() as f32).powf(2.0);
         let mut e: f32 = 0.;
         for j in c1.len()..c1.len()+c2.len(){
            e += kernel[j][i];
         }
         e /= c2.len() as f32;
         
         
         if (g-2.*s) > (w-2.*e){
            
            c2.push(c1[i]);
            let b : f32 = c1[i];
            c1.retain(|x| *x != b);  
            break;
            
         }
      }
      
      for i in 0..c2.len(){
         
         let mut g: f32 = 0.;
         for j in 0..c1.len(){
            for k in 0..c1.len(){
               g += kernel[j][k];
            }
         }
         g /= (c1.len() as f32).powf(2.0);
         let mut s: f32 = 0.;
         for j in 0..c1.len(){
            s += kernel[j][i+c1.len()];
         }
         s /= c1.len() as f32;
         
         let mut w: f32 = 0.;
         for j in c1.len()..c1.len()+c2.len(){
            for k in c1.len()..c1.len()+c2.len(){
               w += kernel[j][k];
            }
         }
         w /= (c2.len() as f32).powf(2.0);
         let mut e: f32 = 0.;
         for j in c1.len()..c1.len()+c2.len(){
            e += kernel[j][i+c1.len()];
         }
         e /= c2.len() as f32;         
         
         if (g-2.*s) < (w-2.*e){
     
            c1.push(c2[i]);
            let b : f32 = c2[i];
            c2.retain(|x| *x != b);
            break;
         }
      }
      
      println!("{:?}", c1);
      println!("{:?}", c2);
      
      let mut v1 : Vec<f32> = Vec::new();
      let mut v2 : Vec<f32> = Vec::new();
      v1.push(c1.len() as f32);
      v2.push(c2.len() as f32);
      
    let mut a = 0;
    loop{
      for i in 0..c1.len(){
      
         let mut g: f32 = 0.;
         for j in 0..c1.len(){
            for k in 0..c1.len(){
               g += kernel[j][k];
            }
         }
         g /= (c1.len() as f32).powf(2.0);
         let mut s: f32 = 0.;
         for j in 0..c1.len(){
            s += kernel[j][i];
         }
         s /= c1.len() as f32;
         
         let mut w: f32 = 0.;
         for j in c1.len()..c1.len()+c2.len(){
            for k in c1.len()..c1.len()+c2.len(){
               w += kernel[j][k];
            }
         }
         w /= (c2.len() as f32).powf(2.0);
         let mut e: f32 = 0.;
         for j in c1.len()..c1.len()+c2.len(){
            e += kernel[j][i];
         }
         e /= c2.len() as f32;
         
         
         if (g-2.*s) > (w-2.*e){
            
            c2.push(c1[i]);
            let b : f32 = c1[i];
            c1.retain(|x| *x != b);  
            break;
            
         }
      }
      
      for i in 0..c2.len(){
         
         let mut g: f32 = 0.;
         for j in 0..c1.len(){
            for k in 0..c1.len(){
               g += kernel[j][k];
            }
         }
         g /= (c1.len() as f32).powf(2.0);
         let mut s: f32 = 0.;
         for j in 0..c1.len(){
            s += kernel[j][i+c1.len()];
         }
         s /= c1.len() as f32;
         
         let mut w: f32 = 0.;
         for j in c1.len()..c1.len()+c2.len(){
            for k in c1.len()..c1.len()+c2.len(){
               w += kernel[j][k];
            }
         }
         w /= (c2.len() as f32).powf(2.0);
         let mut e: f32 = 0.;
         for j in c1.len()..c1.len()+c2.len(){
            e += kernel[j][i+c1.len()];
         }
         e /= c2.len() as f32;         
         
         if (g-2.*s) < (w-2.*e){
     
            c1.push(c2[i]);
            let b : f32 = c2[i];
            c2.retain(|x| *x != b);
            break;
         }
      }
      
      println!("{:?}", c1);
      println!("{:?}", c2);
      
      v1.push(c1.len() as f32);
      v2.push(c2.len() as f32);
      a +=1;
      
      if v1[a] == v1[a-1] && v2[a] == v2[a-1]{
       break;
      }
    }
}
