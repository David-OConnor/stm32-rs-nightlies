#[doc = "Register `DTS_OR` reader"]
pub struct R(crate::R<DTS_OR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTS_OR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTS_OR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTS_OR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTS_OR` writer"]
pub struct W(crate::W<DTS_OR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTS_OR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DTS_OR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTS_OR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TS_Op0` reader - TS_Op0"]
pub type TS_OP0_R = crate::BitReader<bool>;
#[doc = "Field `TS_Op0` writer - TS_Op0"]
pub type TS_OP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_Op1` reader - TS_Op1"]
pub type TS_OP1_R = crate::BitReader<bool>;
#[doc = "Field `TS_Op1` writer - TS_Op1"]
pub type TS_OP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_Op2` reader - TS_Op2"]
pub type TS_OP2_R = crate::BitReader<bool>;
#[doc = "Field `TS_Op2` writer - TS_Op2"]
pub type TS_OP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_Op3` reader - TS_Op3"]
pub type TS_OP3_R = crate::BitReader<bool>;
#[doc = "Field `TS_Op3` writer - TS_Op3"]
pub type TS_OP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_Op4` reader - TS_Op4"]
pub type TS_OP4_R = crate::BitReader<bool>;
#[doc = "Field `TS_Op4` writer - TS_Op4"]
pub type TS_OP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_Op5` reader - TS_Op5"]
pub type TS_OP5_R = crate::BitReader<bool>;
#[doc = "Field `TS_Op5` writer - TS_Op5"]
pub type TS_OP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_Op6` reader - TS_Op6"]
pub type TS_OP6_R = crate::BitReader<bool>;
#[doc = "Field `TS_Op6` writer - TS_Op6"]
pub type TS_OP6_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_Op7` reader - TS_Op7"]
pub type TS_OP7_R = crate::BitReader<bool>;
#[doc = "Field `TS_Op7` writer - TS_Op7"]
pub type TS_OP7_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_Op8` reader - TS_Op8"]
pub type TS_OP8_R = crate::BitReader<bool>;
#[doc = "Field `TS_Op8` writer - TS_Op8"]
pub type TS_OP8_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_Op9` reader - TS_Op9"]
pub type TS_OP9_R = crate::BitReader<bool>;
#[doc = "Field `TS_Op9` writer - TS_Op9"]
pub type TS_OP9_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_Op10` reader - TS_Op10"]
pub type TS_OP10_R = crate::BitReader<bool>;
#[doc = "Field `TS_Op10` writer - TS_Op10"]
pub type TS_OP10_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_Op11` reader - TS_Op11"]
pub type TS_OP11_R = crate::BitReader<bool>;
#[doc = "Field `TS_Op11` writer - TS_Op11"]
pub type TS_OP11_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_Op12` reader - TS_Op12"]
pub type TS_OP12_R = crate::BitReader<bool>;
#[doc = "Field `TS_Op12` writer - TS_Op12"]
pub type TS_OP12_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_Op13` reader - TS_Op13"]
pub type TS_OP13_R = crate::BitReader<bool>;
#[doc = "Field `TS_Op13` writer - TS_Op13"]
pub type TS_OP13_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_Op14` reader - TS_Op14"]
pub type TS_OP14_R = crate::BitReader<bool>;
#[doc = "Field `TS_Op14` writer - TS_Op14"]
pub type TS_OP14_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_Op15` reader - TS_Op15"]
pub type TS_OP15_R = crate::BitReader<bool>;
#[doc = "Field `TS_Op15` writer - TS_Op15"]
pub type TS_OP15_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_Op16` reader - TS_Op16"]
pub type TS_OP16_R = crate::BitReader<bool>;
#[doc = "Field `TS_Op16` writer - TS_Op16"]
pub type TS_OP16_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_Op17` reader - TS_Op17"]
pub type TS_OP17_R = crate::BitReader<bool>;
#[doc = "Field `TS_Op17` writer - TS_Op17"]
pub type TS_OP17_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_Op18` reader - TS_Op18"]
pub type TS_OP18_R = crate::BitReader<bool>;
#[doc = "Field `TS_Op18` writer - TS_Op18"]
pub type TS_OP18_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_Op19` reader - TS_Op19"]
pub type TS_OP19_R = crate::BitReader<bool>;
#[doc = "Field `TS_Op19` writer - TS_Op19"]
pub type TS_OP19_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_Op20` reader - TS_Op20"]
pub type TS_OP20_R = crate::BitReader<bool>;
#[doc = "Field `TS_Op20` writer - TS_Op20"]
pub type TS_OP20_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_Op21` reader - TS_Op21"]
pub type TS_OP21_R = crate::BitReader<bool>;
#[doc = "Field `TS_Op21` writer - TS_Op21"]
pub type TS_OP21_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_Op22` reader - TS_Op22"]
pub type TS_OP22_R = crate::BitReader<bool>;
#[doc = "Field `TS_Op22` writer - TS_Op22"]
pub type TS_OP22_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_Op23` reader - TS_Op23"]
pub type TS_OP23_R = crate::BitReader<bool>;
#[doc = "Field `TS_Op23` writer - TS_Op23"]
pub type TS_OP23_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_Op24` reader - TS_Op24"]
pub type TS_OP24_R = crate::BitReader<bool>;
#[doc = "Field `TS_Op24` writer - TS_Op24"]
pub type TS_OP24_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_Op25` reader - TS_Op25"]
pub type TS_OP25_R = crate::BitReader<bool>;
#[doc = "Field `TS_Op25` writer - TS_Op25"]
pub type TS_OP25_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_Op26` reader - TS_Op26"]
pub type TS_OP26_R = crate::BitReader<bool>;
#[doc = "Field `TS_Op26` writer - TS_Op26"]
pub type TS_OP26_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_Op27` reader - TS_Op27"]
pub type TS_OP27_R = crate::BitReader<bool>;
#[doc = "Field `TS_Op27` writer - TS_Op27"]
pub type TS_OP27_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_Op28` reader - TS_Op28"]
pub type TS_OP28_R = crate::BitReader<bool>;
#[doc = "Field `TS_Op28` writer - TS_Op28"]
pub type TS_OP28_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_Op29` reader - TS_Op29"]
pub type TS_OP29_R = crate::BitReader<bool>;
#[doc = "Field `TS_Op29` writer - TS_Op29"]
pub type TS_OP29_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_Op30` reader - TS_Op30"]
pub type TS_OP30_R = crate::BitReader<bool>;
#[doc = "Field `TS_Op30` writer - TS_Op30"]
pub type TS_OP30_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_Op31` reader - TS_Op31"]
pub type TS_OP31_R = crate::BitReader<bool>;
#[doc = "Field `TS_Op31` writer - TS_Op31"]
pub type TS_OP31_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TS_Op0"]
    #[inline(always)]
    pub fn ts_op0(&self) -> TS_OP0_R {
        TS_OP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TS_Op1"]
    #[inline(always)]
    pub fn ts_op1(&self) -> TS_OP1_R {
        TS_OP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TS_Op2"]
    #[inline(always)]
    pub fn ts_op2(&self) -> TS_OP2_R {
        TS_OP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TS_Op3"]
    #[inline(always)]
    pub fn ts_op3(&self) -> TS_OP3_R {
        TS_OP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TS_Op4"]
    #[inline(always)]
    pub fn ts_op4(&self) -> TS_OP4_R {
        TS_OP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TS_Op5"]
    #[inline(always)]
    pub fn ts_op5(&self) -> TS_OP5_R {
        TS_OP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TS_Op6"]
    #[inline(always)]
    pub fn ts_op6(&self) -> TS_OP6_R {
        TS_OP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TS_Op7"]
    #[inline(always)]
    pub fn ts_op7(&self) -> TS_OP7_R {
        TS_OP7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TS_Op8"]
    #[inline(always)]
    pub fn ts_op8(&self) -> TS_OP8_R {
        TS_OP8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TS_Op9"]
    #[inline(always)]
    pub fn ts_op9(&self) -> TS_OP9_R {
        TS_OP9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TS_Op10"]
    #[inline(always)]
    pub fn ts_op10(&self) -> TS_OP10_R {
        TS_OP10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TS_Op11"]
    #[inline(always)]
    pub fn ts_op11(&self) -> TS_OP11_R {
        TS_OP11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TS_Op12"]
    #[inline(always)]
    pub fn ts_op12(&self) -> TS_OP12_R {
        TS_OP12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TS_Op13"]
    #[inline(always)]
    pub fn ts_op13(&self) -> TS_OP13_R {
        TS_OP13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TS_Op14"]
    #[inline(always)]
    pub fn ts_op14(&self) -> TS_OP14_R {
        TS_OP14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TS_Op15"]
    #[inline(always)]
    pub fn ts_op15(&self) -> TS_OP15_R {
        TS_OP15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TS_Op16"]
    #[inline(always)]
    pub fn ts_op16(&self) -> TS_OP16_R {
        TS_OP16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TS_Op17"]
    #[inline(always)]
    pub fn ts_op17(&self) -> TS_OP17_R {
        TS_OP17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TS_Op18"]
    #[inline(always)]
    pub fn ts_op18(&self) -> TS_OP18_R {
        TS_OP18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TS_Op19"]
    #[inline(always)]
    pub fn ts_op19(&self) -> TS_OP19_R {
        TS_OP19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TS_Op20"]
    #[inline(always)]
    pub fn ts_op20(&self) -> TS_OP20_R {
        TS_OP20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TS_Op21"]
    #[inline(always)]
    pub fn ts_op21(&self) -> TS_OP21_R {
        TS_OP21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - TS_Op22"]
    #[inline(always)]
    pub fn ts_op22(&self) -> TS_OP22_R {
        TS_OP22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TS_Op23"]
    #[inline(always)]
    pub fn ts_op23(&self) -> TS_OP23_R {
        TS_OP23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - TS_Op24"]
    #[inline(always)]
    pub fn ts_op24(&self) -> TS_OP24_R {
        TS_OP24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TS_Op25"]
    #[inline(always)]
    pub fn ts_op25(&self) -> TS_OP25_R {
        TS_OP25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TS_Op26"]
    #[inline(always)]
    pub fn ts_op26(&self) -> TS_OP26_R {
        TS_OP26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TS_Op27"]
    #[inline(always)]
    pub fn ts_op27(&self) -> TS_OP27_R {
        TS_OP27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - TS_Op28"]
    #[inline(always)]
    pub fn ts_op28(&self) -> TS_OP28_R {
        TS_OP28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - TS_Op29"]
    #[inline(always)]
    pub fn ts_op29(&self) -> TS_OP29_R {
        TS_OP29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TS_Op30"]
    #[inline(always)]
    pub fn ts_op30(&self) -> TS_OP30_R {
        TS_OP30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - TS_Op31"]
    #[inline(always)]
    pub fn ts_op31(&self) -> TS_OP31_R {
        TS_OP31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TS_Op0"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op0(&mut self) -> TS_OP0_W<0> {
        TS_OP0_W::new(self)
    }
    #[doc = "Bit 1 - TS_Op1"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op1(&mut self) -> TS_OP1_W<1> {
        TS_OP1_W::new(self)
    }
    #[doc = "Bit 2 - TS_Op2"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op2(&mut self) -> TS_OP2_W<2> {
        TS_OP2_W::new(self)
    }
    #[doc = "Bit 3 - TS_Op3"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op3(&mut self) -> TS_OP3_W<3> {
        TS_OP3_W::new(self)
    }
    #[doc = "Bit 4 - TS_Op4"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op4(&mut self) -> TS_OP4_W<4> {
        TS_OP4_W::new(self)
    }
    #[doc = "Bit 5 - TS_Op5"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op5(&mut self) -> TS_OP5_W<5> {
        TS_OP5_W::new(self)
    }
    #[doc = "Bit 6 - TS_Op6"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op6(&mut self) -> TS_OP6_W<6> {
        TS_OP6_W::new(self)
    }
    #[doc = "Bit 7 - TS_Op7"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op7(&mut self) -> TS_OP7_W<7> {
        TS_OP7_W::new(self)
    }
    #[doc = "Bit 8 - TS_Op8"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op8(&mut self) -> TS_OP8_W<8> {
        TS_OP8_W::new(self)
    }
    #[doc = "Bit 9 - TS_Op9"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op9(&mut self) -> TS_OP9_W<9> {
        TS_OP9_W::new(self)
    }
    #[doc = "Bit 10 - TS_Op10"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op10(&mut self) -> TS_OP10_W<10> {
        TS_OP10_W::new(self)
    }
    #[doc = "Bit 11 - TS_Op11"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op11(&mut self) -> TS_OP11_W<11> {
        TS_OP11_W::new(self)
    }
    #[doc = "Bit 12 - TS_Op12"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op12(&mut self) -> TS_OP12_W<12> {
        TS_OP12_W::new(self)
    }
    #[doc = "Bit 13 - TS_Op13"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op13(&mut self) -> TS_OP13_W<13> {
        TS_OP13_W::new(self)
    }
    #[doc = "Bit 14 - TS_Op14"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op14(&mut self) -> TS_OP14_W<14> {
        TS_OP14_W::new(self)
    }
    #[doc = "Bit 15 - TS_Op15"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op15(&mut self) -> TS_OP15_W<15> {
        TS_OP15_W::new(self)
    }
    #[doc = "Bit 16 - TS_Op16"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op16(&mut self) -> TS_OP16_W<16> {
        TS_OP16_W::new(self)
    }
    #[doc = "Bit 17 - TS_Op17"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op17(&mut self) -> TS_OP17_W<17> {
        TS_OP17_W::new(self)
    }
    #[doc = "Bit 18 - TS_Op18"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op18(&mut self) -> TS_OP18_W<18> {
        TS_OP18_W::new(self)
    }
    #[doc = "Bit 19 - TS_Op19"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op19(&mut self) -> TS_OP19_W<19> {
        TS_OP19_W::new(self)
    }
    #[doc = "Bit 20 - TS_Op20"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op20(&mut self) -> TS_OP20_W<20> {
        TS_OP20_W::new(self)
    }
    #[doc = "Bit 21 - TS_Op21"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op21(&mut self) -> TS_OP21_W<21> {
        TS_OP21_W::new(self)
    }
    #[doc = "Bit 22 - TS_Op22"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op22(&mut self) -> TS_OP22_W<22> {
        TS_OP22_W::new(self)
    }
    #[doc = "Bit 23 - TS_Op23"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op23(&mut self) -> TS_OP23_W<23> {
        TS_OP23_W::new(self)
    }
    #[doc = "Bit 24 - TS_Op24"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op24(&mut self) -> TS_OP24_W<24> {
        TS_OP24_W::new(self)
    }
    #[doc = "Bit 25 - TS_Op25"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op25(&mut self) -> TS_OP25_W<25> {
        TS_OP25_W::new(self)
    }
    #[doc = "Bit 26 - TS_Op26"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op26(&mut self) -> TS_OP26_W<26> {
        TS_OP26_W::new(self)
    }
    #[doc = "Bit 27 - TS_Op27"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op27(&mut self) -> TS_OP27_W<27> {
        TS_OP27_W::new(self)
    }
    #[doc = "Bit 28 - TS_Op28"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op28(&mut self) -> TS_OP28_W<28> {
        TS_OP28_W::new(self)
    }
    #[doc = "Bit 29 - TS_Op29"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op29(&mut self) -> TS_OP29_W<29> {
        TS_OP29_W::new(self)
    }
    #[doc = "Bit 30 - TS_Op30"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op30(&mut self) -> TS_OP30_W<30> {
        TS_OP30_W::new(self)
    }
    #[doc = "Bit 31 - TS_Op31"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op31(&mut self) -> TS_OP31_W<31> {
        TS_OP31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The DTS_OR contains general-purpose option bits.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dts_or](index.html) module"]
pub struct DTS_OR_SPEC;
impl crate::RegisterSpec for DTS_OR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dts_or::R](R) reader structure"]
impl crate::Readable for DTS_OR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dts_or::W](W) writer structure"]
impl crate::Writable for DTS_OR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTS_OR to value 0"]
impl crate::Resettable for DTS_OR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
