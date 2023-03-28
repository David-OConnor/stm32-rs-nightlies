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
#[doc = "Field `TS_OP0` reader - general purpose option bits"]
pub type TS_OP0_R = crate::BitReader<bool>;
#[doc = "Field `TS_OP0` writer - general purpose option bits"]
pub type TS_OP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_OP1` reader - general purpose option bits"]
pub type TS_OP1_R = crate::BitReader<bool>;
#[doc = "Field `TS_OP1` writer - general purpose option bits"]
pub type TS_OP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_OP2` reader - general purpose option bits"]
pub type TS_OP2_R = crate::BitReader<bool>;
#[doc = "Field `TS_OP2` writer - general purpose option bits"]
pub type TS_OP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_OP3` reader - general purpose option bits"]
pub type TS_OP3_R = crate::BitReader<bool>;
#[doc = "Field `TS_OP3` writer - general purpose option bits"]
pub type TS_OP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_OP4` reader - general purpose option bits"]
pub type TS_OP4_R = crate::BitReader<bool>;
#[doc = "Field `TS_OP4` writer - general purpose option bits"]
pub type TS_OP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_OP5` reader - general purpose option bits"]
pub type TS_OP5_R = crate::BitReader<bool>;
#[doc = "Field `TS_OP5` writer - general purpose option bits"]
pub type TS_OP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_OP6` reader - general purpose option bits"]
pub type TS_OP6_R = crate::BitReader<bool>;
#[doc = "Field `TS_OP6` writer - general purpose option bits"]
pub type TS_OP6_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_OP7` reader - general purpose option bits"]
pub type TS_OP7_R = crate::BitReader<bool>;
#[doc = "Field `TS_OP7` writer - general purpose option bits"]
pub type TS_OP7_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_OP8` reader - general purpose option bits"]
pub type TS_OP8_R = crate::BitReader<bool>;
#[doc = "Field `TS_OP8` writer - general purpose option bits"]
pub type TS_OP8_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_OP9` reader - general purpose option bits"]
pub type TS_OP9_R = crate::BitReader<bool>;
#[doc = "Field `TS_OP9` writer - general purpose option bits"]
pub type TS_OP9_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_OP10` reader - general purpose option bits"]
pub type TS_OP10_R = crate::BitReader<bool>;
#[doc = "Field `TS_OP10` writer - general purpose option bits"]
pub type TS_OP10_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_OP11` reader - general purpose option bits"]
pub type TS_OP11_R = crate::BitReader<bool>;
#[doc = "Field `TS_OP11` writer - general purpose option bits"]
pub type TS_OP11_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_OP12` reader - general purpose option bits"]
pub type TS_OP12_R = crate::BitReader<bool>;
#[doc = "Field `TS_OP12` writer - general purpose option bits"]
pub type TS_OP12_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_OP13` reader - general purpose option bits"]
pub type TS_OP13_R = crate::BitReader<bool>;
#[doc = "Field `TS_OP13` writer - general purpose option bits"]
pub type TS_OP13_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_OP14` reader - general purpose option bits"]
pub type TS_OP14_R = crate::BitReader<bool>;
#[doc = "Field `TS_OP14` writer - general purpose option bits"]
pub type TS_OP14_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_OP15` reader - general purpose option bits"]
pub type TS_OP15_R = crate::BitReader<bool>;
#[doc = "Field `TS_OP15` writer - general purpose option bits"]
pub type TS_OP15_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_OP16` reader - general purpose option bits"]
pub type TS_OP16_R = crate::BitReader<bool>;
#[doc = "Field `TS_OP16` writer - general purpose option bits"]
pub type TS_OP16_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_OP17` reader - general purpose option bits"]
pub type TS_OP17_R = crate::BitReader<bool>;
#[doc = "Field `TS_OP17` writer - general purpose option bits"]
pub type TS_OP17_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_OP18` reader - general purpose option bits"]
pub type TS_OP18_R = crate::BitReader<bool>;
#[doc = "Field `TS_OP18` writer - general purpose option bits"]
pub type TS_OP18_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_OP19` reader - general purpose option bits"]
pub type TS_OP19_R = crate::BitReader<bool>;
#[doc = "Field `TS_OP19` writer - general purpose option bits"]
pub type TS_OP19_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_OP20` reader - general purpose option bits"]
pub type TS_OP20_R = crate::BitReader<bool>;
#[doc = "Field `TS_OP20` writer - general purpose option bits"]
pub type TS_OP20_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_OP21` reader - general purpose option bits"]
pub type TS_OP21_R = crate::BitReader<bool>;
#[doc = "Field `TS_OP21` writer - general purpose option bits"]
pub type TS_OP21_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_OP22` reader - general purpose option bits"]
pub type TS_OP22_R = crate::BitReader<bool>;
#[doc = "Field `TS_OP22` writer - general purpose option bits"]
pub type TS_OP22_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_OP23` reader - general purpose option bits"]
pub type TS_OP23_R = crate::BitReader<bool>;
#[doc = "Field `TS_OP23` writer - general purpose option bits"]
pub type TS_OP23_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_OP24` reader - general purpose option bits"]
pub type TS_OP24_R = crate::BitReader<bool>;
#[doc = "Field `TS_OP24` writer - general purpose option bits"]
pub type TS_OP24_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_OP25` reader - general purpose option bits"]
pub type TS_OP25_R = crate::BitReader<bool>;
#[doc = "Field `TS_OP25` writer - general purpose option bits"]
pub type TS_OP25_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_OP26` reader - general purpose option bits"]
pub type TS_OP26_R = crate::BitReader<bool>;
#[doc = "Field `TS_OP26` writer - general purpose option bits"]
pub type TS_OP26_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_OP27` reader - general purpose option bits"]
pub type TS_OP27_R = crate::BitReader<bool>;
#[doc = "Field `TS_OP27` writer - general purpose option bits"]
pub type TS_OP27_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_OP28` reader - general purpose option bits"]
pub type TS_OP28_R = crate::BitReader<bool>;
#[doc = "Field `TS_OP28` writer - general purpose option bits"]
pub type TS_OP28_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_OP29` reader - general purpose option bits"]
pub type TS_OP29_R = crate::BitReader<bool>;
#[doc = "Field `TS_OP29` writer - general purpose option bits"]
pub type TS_OP29_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_OP30` reader - general purpose option bits"]
pub type TS_OP30_R = crate::BitReader<bool>;
#[doc = "Field `TS_OP30` writer - general purpose option bits"]
pub type TS_OP30_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
#[doc = "Field `TS_OP31` reader - general purpose option bits"]
pub type TS_OP31_R = crate::BitReader<bool>;
#[doc = "Field `TS_OP31` writer - general purpose option bits"]
pub type TS_OP31_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTS_OR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - general purpose option bits"]
    #[inline(always)]
    pub fn ts_op0(&self) -> TS_OP0_R {
        TS_OP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - general purpose option bits"]
    #[inline(always)]
    pub fn ts_op1(&self) -> TS_OP1_R {
        TS_OP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - general purpose option bits"]
    #[inline(always)]
    pub fn ts_op2(&self) -> TS_OP2_R {
        TS_OP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - general purpose option bits"]
    #[inline(always)]
    pub fn ts_op3(&self) -> TS_OP3_R {
        TS_OP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - general purpose option bits"]
    #[inline(always)]
    pub fn ts_op4(&self) -> TS_OP4_R {
        TS_OP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - general purpose option bits"]
    #[inline(always)]
    pub fn ts_op5(&self) -> TS_OP5_R {
        TS_OP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - general purpose option bits"]
    #[inline(always)]
    pub fn ts_op6(&self) -> TS_OP6_R {
        TS_OP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - general purpose option bits"]
    #[inline(always)]
    pub fn ts_op7(&self) -> TS_OP7_R {
        TS_OP7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - general purpose option bits"]
    #[inline(always)]
    pub fn ts_op8(&self) -> TS_OP8_R {
        TS_OP8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - general purpose option bits"]
    #[inline(always)]
    pub fn ts_op9(&self) -> TS_OP9_R {
        TS_OP9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - general purpose option bits"]
    #[inline(always)]
    pub fn ts_op10(&self) -> TS_OP10_R {
        TS_OP10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - general purpose option bits"]
    #[inline(always)]
    pub fn ts_op11(&self) -> TS_OP11_R {
        TS_OP11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - general purpose option bits"]
    #[inline(always)]
    pub fn ts_op12(&self) -> TS_OP12_R {
        TS_OP12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - general purpose option bits"]
    #[inline(always)]
    pub fn ts_op13(&self) -> TS_OP13_R {
        TS_OP13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - general purpose option bits"]
    #[inline(always)]
    pub fn ts_op14(&self) -> TS_OP14_R {
        TS_OP14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - general purpose option bits"]
    #[inline(always)]
    pub fn ts_op15(&self) -> TS_OP15_R {
        TS_OP15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - general purpose option bits"]
    #[inline(always)]
    pub fn ts_op16(&self) -> TS_OP16_R {
        TS_OP16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - general purpose option bits"]
    #[inline(always)]
    pub fn ts_op17(&self) -> TS_OP17_R {
        TS_OP17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - general purpose option bits"]
    #[inline(always)]
    pub fn ts_op18(&self) -> TS_OP18_R {
        TS_OP18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - general purpose option bits"]
    #[inline(always)]
    pub fn ts_op19(&self) -> TS_OP19_R {
        TS_OP19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - general purpose option bits"]
    #[inline(always)]
    pub fn ts_op20(&self) -> TS_OP20_R {
        TS_OP20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - general purpose option bits"]
    #[inline(always)]
    pub fn ts_op21(&self) -> TS_OP21_R {
        TS_OP21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - general purpose option bits"]
    #[inline(always)]
    pub fn ts_op22(&self) -> TS_OP22_R {
        TS_OP22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - general purpose option bits"]
    #[inline(always)]
    pub fn ts_op23(&self) -> TS_OP23_R {
        TS_OP23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - general purpose option bits"]
    #[inline(always)]
    pub fn ts_op24(&self) -> TS_OP24_R {
        TS_OP24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - general purpose option bits"]
    #[inline(always)]
    pub fn ts_op25(&self) -> TS_OP25_R {
        TS_OP25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - general purpose option bits"]
    #[inline(always)]
    pub fn ts_op26(&self) -> TS_OP26_R {
        TS_OP26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - general purpose option bits"]
    #[inline(always)]
    pub fn ts_op27(&self) -> TS_OP27_R {
        TS_OP27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - general purpose option bits"]
    #[inline(always)]
    pub fn ts_op28(&self) -> TS_OP28_R {
        TS_OP28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - general purpose option bits"]
    #[inline(always)]
    pub fn ts_op29(&self) -> TS_OP29_R {
        TS_OP29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - general purpose option bits"]
    #[inline(always)]
    pub fn ts_op30(&self) -> TS_OP30_R {
        TS_OP30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - general purpose option bits"]
    #[inline(always)]
    pub fn ts_op31(&self) -> TS_OP31_R {
        TS_OP31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - general purpose option bits"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op0(&mut self) -> TS_OP0_W<0> {
        TS_OP0_W::new(self)
    }
    #[doc = "Bit 1 - general purpose option bits"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op1(&mut self) -> TS_OP1_W<1> {
        TS_OP1_W::new(self)
    }
    #[doc = "Bit 2 - general purpose option bits"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op2(&mut self) -> TS_OP2_W<2> {
        TS_OP2_W::new(self)
    }
    #[doc = "Bit 3 - general purpose option bits"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op3(&mut self) -> TS_OP3_W<3> {
        TS_OP3_W::new(self)
    }
    #[doc = "Bit 4 - general purpose option bits"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op4(&mut self) -> TS_OP4_W<4> {
        TS_OP4_W::new(self)
    }
    #[doc = "Bit 5 - general purpose option bits"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op5(&mut self) -> TS_OP5_W<5> {
        TS_OP5_W::new(self)
    }
    #[doc = "Bit 6 - general purpose option bits"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op6(&mut self) -> TS_OP6_W<6> {
        TS_OP6_W::new(self)
    }
    #[doc = "Bit 7 - general purpose option bits"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op7(&mut self) -> TS_OP7_W<7> {
        TS_OP7_W::new(self)
    }
    #[doc = "Bit 8 - general purpose option bits"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op8(&mut self) -> TS_OP8_W<8> {
        TS_OP8_W::new(self)
    }
    #[doc = "Bit 9 - general purpose option bits"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op9(&mut self) -> TS_OP9_W<9> {
        TS_OP9_W::new(self)
    }
    #[doc = "Bit 10 - general purpose option bits"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op10(&mut self) -> TS_OP10_W<10> {
        TS_OP10_W::new(self)
    }
    #[doc = "Bit 11 - general purpose option bits"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op11(&mut self) -> TS_OP11_W<11> {
        TS_OP11_W::new(self)
    }
    #[doc = "Bit 12 - general purpose option bits"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op12(&mut self) -> TS_OP12_W<12> {
        TS_OP12_W::new(self)
    }
    #[doc = "Bit 13 - general purpose option bits"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op13(&mut self) -> TS_OP13_W<13> {
        TS_OP13_W::new(self)
    }
    #[doc = "Bit 14 - general purpose option bits"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op14(&mut self) -> TS_OP14_W<14> {
        TS_OP14_W::new(self)
    }
    #[doc = "Bit 15 - general purpose option bits"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op15(&mut self) -> TS_OP15_W<15> {
        TS_OP15_W::new(self)
    }
    #[doc = "Bit 16 - general purpose option bits"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op16(&mut self) -> TS_OP16_W<16> {
        TS_OP16_W::new(self)
    }
    #[doc = "Bit 17 - general purpose option bits"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op17(&mut self) -> TS_OP17_W<17> {
        TS_OP17_W::new(self)
    }
    #[doc = "Bit 18 - general purpose option bits"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op18(&mut self) -> TS_OP18_W<18> {
        TS_OP18_W::new(self)
    }
    #[doc = "Bit 19 - general purpose option bits"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op19(&mut self) -> TS_OP19_W<19> {
        TS_OP19_W::new(self)
    }
    #[doc = "Bit 20 - general purpose option bits"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op20(&mut self) -> TS_OP20_W<20> {
        TS_OP20_W::new(self)
    }
    #[doc = "Bit 21 - general purpose option bits"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op21(&mut self) -> TS_OP21_W<21> {
        TS_OP21_W::new(self)
    }
    #[doc = "Bit 22 - general purpose option bits"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op22(&mut self) -> TS_OP22_W<22> {
        TS_OP22_W::new(self)
    }
    #[doc = "Bit 23 - general purpose option bits"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op23(&mut self) -> TS_OP23_W<23> {
        TS_OP23_W::new(self)
    }
    #[doc = "Bit 24 - general purpose option bits"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op24(&mut self) -> TS_OP24_W<24> {
        TS_OP24_W::new(self)
    }
    #[doc = "Bit 25 - general purpose option bits"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op25(&mut self) -> TS_OP25_W<25> {
        TS_OP25_W::new(self)
    }
    #[doc = "Bit 26 - general purpose option bits"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op26(&mut self) -> TS_OP26_W<26> {
        TS_OP26_W::new(self)
    }
    #[doc = "Bit 27 - general purpose option bits"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op27(&mut self) -> TS_OP27_W<27> {
        TS_OP27_W::new(self)
    }
    #[doc = "Bit 28 - general purpose option bits"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op28(&mut self) -> TS_OP28_W<28> {
        TS_OP28_W::new(self)
    }
    #[doc = "Bit 29 - general purpose option bits"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op29(&mut self) -> TS_OP29_W<29> {
        TS_OP29_W::new(self)
    }
    #[doc = "Bit 30 - general purpose option bits"]
    #[inline(always)]
    #[must_use]
    pub fn ts_op30(&mut self) -> TS_OP30_W<30> {
        TS_OP30_W::new(self)
    }
    #[doc = "Bit 31 - general purpose option bits"]
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
#[doc = "Temperature sensor option register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dts_or](index.html) module"]
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
