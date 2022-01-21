use crate::frame::mmm::cost_model::CostModel;
pub fn models() -> Vec<(&'static str, CostModel)> {
vec!(
("generic_f32_4x4", CostModel { mr: 4, nr: 4,
intercept: 0.0000005997218162178767,
coef: vec!(3.92514846780001e-7, 2.3141830304366058e-8, -1.4017321294978934e-7, 2.080005652262695e-10, -9.646537551761683e-8, -2.3948394277493397e-11, 9.304760937444612e-8, 1.153142541621969e-7, 2.1377992226668027e-8, 1.4400428216184766e-8, -1.7133627925747308e-7),
}),
("armv7neon_mmm_f32_8x4_cortexa7", CostModel { mr: 8, nr: 4,
intercept: 0.00000043997766404589743,
coef: vec!(3.4210795017203023e-7, 3.399842648420204e-8, 3.932548213411596e-8, -3.165524397837049e-10, 7.355429859648475e-8, -1.2491800807727786e-10, 1.1959135643116404e-7, 1.675920102394434e-7, 1.592606787814862e-8, 1.6995211886919324e-8, -2.356652593958311e-7),
}),
("armv7neon_mmm_f32_8x6_cortexa7", CostModel { mr: 8, nr: 6,
intercept: 0.0000005876633512704024,
coef: vec!(3.7690877699529877e-7, 4.7156506639609995e-8, -9.039964165687446e-8, -3.238694517633661e-10, -2.3800207025646673e-8, -6.93223844714082e-10, 9.78910041110515e-8, 1.4245886414641867e-7, 1.781933647339224e-8, 1.6793107162725936e-8, -2.595080228537459e-7),
}),
("armv7neon_mmm_f32_8x4_cortexa9", CostModel { mr: 8, nr: 4,
intercept: 0.0000004701967593336351,
coef: vec!(3.4101752189842225e-7, 3.476733281601892e-8, 1.9477092426167262e-8, -3.8532301888313265e-10, 5.68742168353679e-8, -7.910448579629399e-11, 1.1803615774408084e-7, 1.6885296897977545e-7, 1.5440235439727344e-8, 1.602186280854304e-8, -2.3273081580998736e-7),
}),
("armv7neon_mmm_f32_8x6_cortexa9", CostModel { mr: 8, nr: 6,
intercept: 0.0000005756261170625097,
coef: vec!(3.774547108001729e-7, 4.7949123624347394e-8, -6.191310737257866e-8, -3.710414821081831e-10, -5.918973525570181e-8, -7.712537494572737e-10, 9.087136214735243e-8, 1.33919471061743e-7, 1.7943290422681673e-8, 1.636575629331742e-8, -2.5221744142484213e-7),
}),
("armv7neon_mmm_f32_8x4_generic", CostModel { mr: 8, nr: 4,
intercept: 0.00000021006113472677703,
coef: vec!(3.496681324155415e-7, 3.256912122694908e-8, 3.418479081869369e-7, -1.30918464599891e-9, 2.9090662279640174e-7, -2.237401533855104e-10, 1.2845251553246495e-7, 1.5602316974829717e-7, 1.442466573382405e-8, 2.379012155758417e-8, -2.3672813783359376e-7),
}),
("armv7neon_mmm_f32_8x6_generic", CostModel { mr: 8, nr: 6,
intercept: -0.00000013775189790457743,
coef: vec!(4.057102917094664e-7, 4.602761285441833e-8, 7.829461604208215e-7, -2.343793649891909e-9, 7.2017120141472e-7, -2.3631272981789076e-9, 9.448281699218877e-8, 9.89002762983451e-8, 1.7212483757032855e-8, 2.2557849337592347e-8, -2.2921883712545805e-7),
}),
)}
