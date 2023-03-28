#[doc = "Register `GPIOK_BRR` writer"]
pub struct W(crate::W<GPIOK_BRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOK_BRR_SPEC>;
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
impl From<crate::W<GPIOK_BRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOK_BRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BR0` writer - BR0"]
pub type BR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOK_BRR_SPEC, bool, O>;
#[doc = "Field `BR1` writer - BR1"]
pub type BR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOK_BRR_SPEC, bool, O>;
#[doc = "Field `BR2` writer - BR2"]
pub type BR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOK_BRR_SPEC, bool, O>;
#[doc = "Field `BR3` writer - BR3"]
pub type BR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOK_BRR_SPEC, bool, O>;
#[doc = "Field `BR4` writer - BR4"]
pub type BR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOK_BRR_SPEC, bool, O>;
#[doc = "Field `BR5` writer - BR5"]
pub type BR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOK_BRR_SPEC, bool, O>;
#[doc = "Field `BR6` writer - BR6"]
pub type BR6_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOK_BRR_SPEC, bool, O>;
#[doc = "Field `BR7` writer - BR7"]
pub type BR7_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOK_BRR_SPEC, bool, O>;
#[doc = "Field `BR8` writer - BR8"]
pub type BR8_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOK_BRR_SPEC, bool, O>;
#[doc = "Field `BR9` writer - BR9"]
pub type BR9_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOK_BRR_SPEC, bool, O>;
#[doc = "Field `BR10` writer - BR10"]
pub type BR10_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOK_BRR_SPEC, bool, O>;
#[doc = "Field `BR11` writer - BR11"]
pub type BR11_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOK_BRR_SPEC, bool, O>;
#[doc = "Field `BR12` writer - BR12"]
pub type BR12_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOK_BRR_SPEC, bool, O>;
#[doc = "Field `BR13` writer - BR13"]
pub type BR13_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOK_BRR_SPEC, bool, O>;
#[doc = "Field `BR14` writer - BR14"]
pub type BR14_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOK_BRR_SPEC, bool, O>;
#[doc = "Field `BR15` writer - BR15"]
pub type BR15_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOK_BRR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - BR0"]
    #[inline(always)]
    #[must_use]
    pub fn br0(&mut self) -> BR0_W<0> {
        BR0_W::new(self)
    }
    #[doc = "Bit 1 - BR1"]
    #[inline(always)]
    #[must_use]
    pub fn br1(&mut self) -> BR1_W<1> {
        BR1_W::new(self)
    }
    #[doc = "Bit 2 - BR2"]
    #[inline(always)]
    #[must_use]
    pub fn br2(&mut self) -> BR2_W<2> {
        BR2_W::new(self)
    }
    #[doc = "Bit 3 - BR3"]
    #[inline(always)]
    #[must_use]
    pub fn br3(&mut self) -> BR3_W<3> {
        BR3_W::new(self)
    }
    #[doc = "Bit 4 - BR4"]
    #[inline(always)]
    #[must_use]
    pub fn br4(&mut self) -> BR4_W<4> {
        BR4_W::new(self)
    }
    #[doc = "Bit 5 - BR5"]
    #[inline(always)]
    #[must_use]
    pub fn br5(&mut self) -> BR5_W<5> {
        BR5_W::new(self)
    }
    #[doc = "Bit 6 - BR6"]
    #[inline(always)]
    #[must_use]
    pub fn br6(&mut self) -> BR6_W<6> {
        BR6_W::new(self)
    }
    #[doc = "Bit 7 - BR7"]
    #[inline(always)]
    #[must_use]
    pub fn br7(&mut self) -> BR7_W<7> {
        BR7_W::new(self)
    }
    #[doc = "Bit 8 - BR8"]
    #[inline(always)]
    #[must_use]
    pub fn br8(&mut self) -> BR8_W<8> {
        BR8_W::new(self)
    }
    #[doc = "Bit 9 - BR9"]
    #[inline(always)]
    #[must_use]
    pub fn br9(&mut self) -> BR9_W<9> {
        BR9_W::new(self)
    }
    #[doc = "Bit 10 - BR10"]
    #[inline(always)]
    #[must_use]
    pub fn br10(&mut self) -> BR10_W<10> {
        BR10_W::new(self)
    }
    #[doc = "Bit 11 - BR11"]
    #[inline(always)]
    #[must_use]
    pub fn br11(&mut self) -> BR11_W<11> {
        BR11_W::new(self)
    }
    #[doc = "Bit 12 - BR12"]
    #[inline(always)]
    #[must_use]
    pub fn br12(&mut self) -> BR12_W<12> {
        BR12_W::new(self)
    }
    #[doc = "Bit 13 - BR13"]
    #[inline(always)]
    #[must_use]
    pub fn br13(&mut self) -> BR13_W<13> {
        BR13_W::new(self)
    }
    #[doc = "Bit 14 - BR14"]
    #[inline(always)]
    #[must_use]
    pub fn br14(&mut self) -> BR14_W<14> {
        BR14_W::new(self)
    }
    #[doc = "Bit 15 - BR15"]
    #[inline(always)]
    #[must_use]
    pub fn br15(&mut self) -> BR15_W<15> {
        BR15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port bit reset register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiok_brr](index.html) module"]
pub struct GPIOK_BRR_SPEC;
impl crate::RegisterSpec for GPIOK_BRR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gpiok_brr::W](W) writer structure"]
impl crate::Writable for GPIOK_BRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIOK_BRR to value 0"]
impl crate::Resettable for GPIOK_BRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
