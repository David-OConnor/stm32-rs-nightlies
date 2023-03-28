#[doc = "Register `BSRR` writer"]
pub struct W(crate::W<BSRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BSRR_SPEC>;
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
impl From<crate::W<BSRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BSRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Set bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS0W_AW {
    #[doc = "1: Sets the corresponding ODRx bit"]
    Set = 1,
}
impl From<BS0W_AW> for bool {
    #[inline(always)]
    fn from(variant: BS0W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS0` writer - Set bit 0"]
pub type BS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSRR_SPEC, BS0W_AW, O>;
impl<'a, const O: u8> BS0_W<'a, O> {
    #[doc = "Sets the corresponding ODRx bit"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BS0W_AW::Set)
    }
}
#[doc = "Field `BS1` writer - Set bit 1"]
pub use BS0_W as BS1_W;
#[doc = "Field `BS2` writer - Set bit 1"]
pub use BS0_W as BS2_W;
#[doc = "Field `BS3` writer - Set bit 3"]
pub use BS0_W as BS3_W;
#[doc = "Field `BS4` writer - Set bit 4"]
pub use BS0_W as BS4_W;
#[doc = "Field `BS5` writer - Set bit 5"]
pub use BS0_W as BS5_W;
#[doc = "Field `BS6` writer - Set bit 6"]
pub use BS0_W as BS6_W;
#[doc = "Field `BS7` writer - Set bit 7"]
pub use BS0_W as BS7_W;
#[doc = "Field `BS8` writer - Set bit 8"]
pub use BS0_W as BS8_W;
#[doc = "Field `BS9` writer - Set bit 9"]
pub use BS0_W as BS9_W;
#[doc = "Field `BS10` writer - Set bit 10"]
pub use BS0_W as BS10_W;
#[doc = "Field `BS11` writer - Set bit 11"]
pub use BS0_W as BS11_W;
#[doc = "Field `BS12` writer - Set bit 12"]
pub use BS0_W as BS12_W;
#[doc = "Field `BS13` writer - Set bit 13"]
pub use BS0_W as BS13_W;
#[doc = "Field `BS14` writer - Set bit 14"]
pub use BS0_W as BS14_W;
#[doc = "Field `BS15` writer - Set bit 15"]
pub use BS0_W as BS15_W;
#[doc = "Reset bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR0W_AW {
    #[doc = "1: Resets the corresponding ODRx bit"]
    Reset = 1,
}
impl From<BR0W_AW> for bool {
    #[inline(always)]
    fn from(variant: BR0W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR0` writer - Reset bit 0"]
pub type BR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSRR_SPEC, BR0W_AW, O>;
impl<'a, const O: u8> BR0_W<'a, O> {
    #[doc = "Resets the corresponding ODRx bit"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR0W_AW::Reset)
    }
}
#[doc = "Field `BR1` writer - Reset bit 1"]
pub use BR0_W as BR1_W;
#[doc = "Field `BR2` writer - Reset bit 2"]
pub use BR0_W as BR2_W;
#[doc = "Field `BR3` writer - Reset bit 3"]
pub use BR0_W as BR3_W;
#[doc = "Field `BR4` writer - Reset bit 4"]
pub use BR0_W as BR4_W;
#[doc = "Field `BR5` writer - Reset bit 5"]
pub use BR0_W as BR5_W;
#[doc = "Field `BR6` writer - Reset bit 6"]
pub use BR0_W as BR6_W;
#[doc = "Field `BR7` writer - Reset bit 7"]
pub use BR0_W as BR7_W;
#[doc = "Field `BR8` writer - Reset bit 8"]
pub use BR0_W as BR8_W;
#[doc = "Field `BR9` writer - Reset bit 9"]
pub use BR0_W as BR9_W;
#[doc = "Field `BR10` writer - Reset bit 10"]
pub use BR0_W as BR10_W;
#[doc = "Field `BR11` writer - Reset bit 11"]
pub use BR0_W as BR11_W;
#[doc = "Field `BR12` writer - Reset bit 12"]
pub use BR0_W as BR12_W;
#[doc = "Field `BR13` writer - Reset bit 13"]
pub use BR0_W as BR13_W;
#[doc = "Field `BR14` writer - Reset bit 14"]
pub use BR0_W as BR14_W;
#[doc = "Field `BR15` writer - Reset bit 15"]
pub use BR0_W as BR15_W;
impl W {
    #[doc = "Bit 0 - Set bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn bs0(&mut self) -> BS0_W<0> {
        BS0_W::new(self)
    }
    #[doc = "Bit 1 - Set bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn bs1(&mut self) -> BS1_W<1> {
        BS1_W::new(self)
    }
    #[doc = "Bit 2 - Set bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn bs2(&mut self) -> BS2_W<2> {
        BS2_W::new(self)
    }
    #[doc = "Bit 3 - Set bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn bs3(&mut self) -> BS3_W<3> {
        BS3_W::new(self)
    }
    #[doc = "Bit 4 - Set bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn bs4(&mut self) -> BS4_W<4> {
        BS4_W::new(self)
    }
    #[doc = "Bit 5 - Set bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn bs5(&mut self) -> BS5_W<5> {
        BS5_W::new(self)
    }
    #[doc = "Bit 6 - Set bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn bs6(&mut self) -> BS6_W<6> {
        BS6_W::new(self)
    }
    #[doc = "Bit 7 - Set bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn bs7(&mut self) -> BS7_W<7> {
        BS7_W::new(self)
    }
    #[doc = "Bit 8 - Set bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn bs8(&mut self) -> BS8_W<8> {
        BS8_W::new(self)
    }
    #[doc = "Bit 9 - Set bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn bs9(&mut self) -> BS9_W<9> {
        BS9_W::new(self)
    }
    #[doc = "Bit 10 - Set bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn bs10(&mut self) -> BS10_W<10> {
        BS10_W::new(self)
    }
    #[doc = "Bit 11 - Set bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn bs11(&mut self) -> BS11_W<11> {
        BS11_W::new(self)
    }
    #[doc = "Bit 12 - Set bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn bs12(&mut self) -> BS12_W<12> {
        BS12_W::new(self)
    }
    #[doc = "Bit 13 - Set bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn bs13(&mut self) -> BS13_W<13> {
        BS13_W::new(self)
    }
    #[doc = "Bit 14 - Set bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn bs14(&mut self) -> BS14_W<14> {
        BS14_W::new(self)
    }
    #[doc = "Bit 15 - Set bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn bs15(&mut self) -> BS15_W<15> {
        BS15_W::new(self)
    }
    #[doc = "Bit 16 - Reset bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn br0(&mut self) -> BR0_W<16> {
        BR0_W::new(self)
    }
    #[doc = "Bit 17 - Reset bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn br1(&mut self) -> BR1_W<17> {
        BR1_W::new(self)
    }
    #[doc = "Bit 18 - Reset bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn br2(&mut self) -> BR2_W<18> {
        BR2_W::new(self)
    }
    #[doc = "Bit 19 - Reset bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn br3(&mut self) -> BR3_W<19> {
        BR3_W::new(self)
    }
    #[doc = "Bit 20 - Reset bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn br4(&mut self) -> BR4_W<20> {
        BR4_W::new(self)
    }
    #[doc = "Bit 21 - Reset bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn br5(&mut self) -> BR5_W<21> {
        BR5_W::new(self)
    }
    #[doc = "Bit 22 - Reset bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn br6(&mut self) -> BR6_W<22> {
        BR6_W::new(self)
    }
    #[doc = "Bit 23 - Reset bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn br7(&mut self) -> BR7_W<23> {
        BR7_W::new(self)
    }
    #[doc = "Bit 24 - Reset bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn br8(&mut self) -> BR8_W<24> {
        BR8_W::new(self)
    }
    #[doc = "Bit 25 - Reset bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn br9(&mut self) -> BR9_W<25> {
        BR9_W::new(self)
    }
    #[doc = "Bit 26 - Reset bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn br10(&mut self) -> BR10_W<26> {
        BR10_W::new(self)
    }
    #[doc = "Bit 27 - Reset bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn br11(&mut self) -> BR11_W<27> {
        BR11_W::new(self)
    }
    #[doc = "Bit 28 - Reset bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn br12(&mut self) -> BR12_W<28> {
        BR12_W::new(self)
    }
    #[doc = "Bit 29 - Reset bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn br13(&mut self) -> BR13_W<29> {
        BR13_W::new(self)
    }
    #[doc = "Bit 30 - Reset bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn br14(&mut self) -> BR14_W<30> {
        BR14_W::new(self)
    }
    #[doc = "Bit 31 - Reset bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn br15(&mut self) -> BR15_W<31> {
        BR15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port bit set/reset register (GPIOn_BSRR)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsrr](index.html) module"]
pub struct BSRR_SPEC;
impl crate::RegisterSpec for BSRR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [bsrr::W](W) writer structure"]
impl crate::Writable for BSRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BSRR to value 0"]
impl crate::Resettable for BSRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
