use crate::{eval_model, Model};

// #[derive(Default)]
// struct Fitter {
//     models: [Model; 30],
//     count: usize,
// }

// impl Fitter {
//     fn new() -> Fitter {
//         let mut result = Fitter {
//             models: Default::default(),
//             count: 1,
//         };
//         result.set_best(0);
//         result
//     }
//
//     fn set_best(&mut self, idx: usize) {
//         assert!(idx < self.count);
//         let model = self.models[idx];
//         if idx != 0 {
//             self.models[0] = model;
//         }
//         let mut idx: usize = 1;
//         {
//             let heads = model.0[15];
//             if heads == 15 {
//                 let m = &mut self.models[idx];
//                 idx += 1;
//                 *m = model;
//                 m.0[15] = 14;
//             } else {
//                 let left = model.0[14];
//                 if heads < left {
//                     let m = &mut self.models[idx];
//                     idx += 1;
//                     *m = model;
//                     m.0[15] = heads + 1;
//                 }
//                 if heads > 0 {
//                     let m = &mut self.models[idx];
//                     idx += 1;
//                     *m = model;
//                     m.0[15] = heads - 1;
//                 }
//             }
//         }
//         let mut tails: usize = 15;
//         while {
//             tails -= 1;
//             tails != 0
//         } {
//             let heads = model.0[tails];
//             let right = model.0[tails + 1];
//             let left = model.0[tails - 1];
//             if heads > right {
//                 let m = &mut self.models[idx];
//                 idx += 1;
//                 *m = model;
//                 m.0[tails] = heads - 1;
//             }
//             if heads < left {
//                 let m = &mut self.models[idx];
//                 idx += 1;
//                 *m = model;
//                 m.0[tails] = heads + 1;
//             }
//         }
//         {
//             let heads = model.0[0];
//             if heads == 0 {
//                 let m = &mut self.models[idx];
//                 idx += 1;
//                 *m = model;
//                 m.0[0] = 1;
//             } else {
//                 let right = model.0[1];
//                 if heads > right {
//                     let m = &mut self.models[idx];
//                     idx += 1;
//                     *m = model;
//                     m.0[0] = heads - 1;
//                 }
//                 if heads < 15 {
//                     let m = &mut self.models[idx];
//                     idx += 1;
//                     *m = model;
//                     m.0[0] = heads + 1;
//                 }
//             }
//         }
//         self.count = idx;
//     }
//
//     fn iter(&self) -> impl Iterator<Item=&Model> {
//         self.models.iter().take(self.count)
//     }
// }

// pub fn evolve_old() -> Model {
//     let mut fitter = Fitter::new();
//     loop {
//         let mut max: u32 = 0;
//         let mut best: usize = 0;
//         for (i, model) in fitter.iter().enumerate() {
//             let current = eval_model(model);
//             println!("{:?} {}", model, current);
//             if current > max {
//                 max = current;
//                 best = i;
//             }
//         }
//         println!("{}", best);
//         if best == 0 {
//             return *fitter.iter().next().unwrap()
//         }
//         fitter.set_best(best);
//     }
// }

// pub fn evolve() -> Model {
//     let mut model = Model::default();
//     let mut best = model;
//     let mut max: u32 = 0;
//     for a in 0..6 {
//         model.0[0] = a;
//         for b in 0..6 {
//             model.0[1] = b;
//             for c in 0..6 {
//                 model.0[2] = c;
//                 for d in 0..6 {
//                     model.0[3] = d;
//                     for e in 0..6 {
//                         model.0[4] = e;
//                         for f in 0..6 {
//                             model.0[5] = f;
//                             let current = eval_model(&model);
//                             if current > max {
//                                 max = current;
//                                 best = model;
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
//     println!("{}", max);
//     return best;
// }

pub fn evolve() -> Model {
    let mut model = Model::default();
    let mut best = model;
    let mut max: u32 = 0;
    // for i in (0..=u16::MAX).rev() {
    for i in (0..=3).rev() {
        model.0 = i;
        let current = eval_model(&model);
        if current > max {
            max = current;
            best = model;
        }
    }
    let report_string = {
        let mut report = best.0;
        let mut string = String::with_capacity(20);
        for _ in 0..4 {
            for _ in 0..4 {
                string.push(if report & 1 == 0 {
                    'T'
                } else {
                    'F'
                });
                report >>= 1;
            }
            string.push('\n');
        }
        string
    };
    println!("{} {}", report_string, max);
    // println!("{:#018b} {}", best.0, max);
    return best;
}
