
pub trait InRange {
    fn in_range(self,begin:Self,end:Self)->bool;
}
impl InRange for f32 {
    fn in_range(self,begin:f32, end:f32)->bool{
        self >=begin && self < end
    }
}

