#[doc = "Register `BRR` reader"]
pub struct R(crate::R<BRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BRR` writer"]
pub struct W(crate::W<BRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRR_SPEC>;
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
impl From<crate::W<BRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BR0` reader - Port A Reset bit 0"]
pub type BR0_R = crate::BitReader<bool>;
#[doc = "Field `BR0` writer - Port A Reset bit 0"]
pub type BR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR1` reader - Port A Reset bit 1"]
pub type BR1_R = crate::BitReader<bool>;
#[doc = "Field `BR1` writer - Port A Reset bit 1"]
pub type BR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR2` reader - Port A Reset bit 2"]
pub type BR2_R = crate::BitReader<bool>;
#[doc = "Field `BR2` writer - Port A Reset bit 2"]
pub type BR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR3` reader - Port A Reset bit 3"]
pub type BR3_R = crate::BitReader<bool>;
#[doc = "Field `BR3` writer - Port A Reset bit 3"]
pub type BR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR4` reader - Port A Reset bit 4"]
pub type BR4_R = crate::BitReader<bool>;
#[doc = "Field `BR4` writer - Port A Reset bit 4"]
pub type BR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR5` reader - Port A Reset bit 5"]
pub type BR5_R = crate::BitReader<bool>;
#[doc = "Field `BR5` writer - Port A Reset bit 5"]
pub type BR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR6` reader - Port A Reset bit 6"]
pub type BR6_R = crate::BitReader<bool>;
#[doc = "Field `BR6` writer - Port A Reset bit 6"]
pub type BR6_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR7` reader - Port A Reset bit 7"]
pub type BR7_R = crate::BitReader<bool>;
#[doc = "Field `BR7` writer - Port A Reset bit 7"]
pub type BR7_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR8` reader - Port A Reset bit 8"]
pub type BR8_R = crate::BitReader<bool>;
#[doc = "Field `BR8` writer - Port A Reset bit 8"]
pub type BR8_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR9` reader - Port A Reset bit 9"]
pub type BR9_R = crate::BitReader<bool>;
#[doc = "Field `BR9` writer - Port A Reset bit 9"]
pub type BR9_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR10` reader - Port A Reset bit 10"]
pub type BR10_R = crate::BitReader<bool>;
#[doc = "Field `BR10` writer - Port A Reset bit 10"]
pub type BR10_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR11` reader - Port A Reset bit 11"]
pub type BR11_R = crate::BitReader<bool>;
#[doc = "Field `BR11` writer - Port A Reset bit 11"]
pub type BR11_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR12` reader - Port A Reset bit 12"]
pub type BR12_R = crate::BitReader<bool>;
#[doc = "Field `BR12` writer - Port A Reset bit 12"]
pub type BR12_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR13` reader - Port A Reset bit 13"]
pub type BR13_R = crate::BitReader<bool>;
#[doc = "Field `BR13` writer - Port A Reset bit 13"]
pub type BR13_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR14` reader - Port A Reset bit 14"]
pub type BR14_R = crate::BitReader<bool>;
#[doc = "Field `BR14` writer - Port A Reset bit 14"]
pub type BR14_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
#[doc = "Field `BR15` reader - Port A Reset bit 15"]
pub type BR15_R = crate::BitReader<bool>;
#[doc = "Field `BR15` writer - Port A Reset bit 15"]
pub type BR15_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Port A Reset bit 0"]
    #[inline(always)]
    pub fn br0(&self) -> BR0_R {
        BR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port A Reset bit 1"]
    #[inline(always)]
    pub fn br1(&self) -> BR1_R {
        BR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port A Reset bit 2"]
    #[inline(always)]
    pub fn br2(&self) -> BR2_R {
        BR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port A Reset bit 3"]
    #[inline(always)]
    pub fn br3(&self) -> BR3_R {
        BR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port A Reset bit 4"]
    #[inline(always)]
    pub fn br4(&self) -> BR4_R {
        BR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port A Reset bit 5"]
    #[inline(always)]
    pub fn br5(&self) -> BR5_R {
        BR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port A Reset bit 6"]
    #[inline(always)]
    pub fn br6(&self) -> BR6_R {
        BR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port A Reset bit 7"]
    #[inline(always)]
    pub fn br7(&self) -> BR7_R {
        BR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port A Reset bit 8"]
    #[inline(always)]
    pub fn br8(&self) -> BR8_R {
        BR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port A Reset bit 9"]
    #[inline(always)]
    pub fn br9(&self) -> BR9_R {
        BR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port A Reset bit 10"]
    #[inline(always)]
    pub fn br10(&self) -> BR10_R {
        BR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port A Reset bit 11"]
    #[inline(always)]
    pub fn br11(&self) -> BR11_R {
        BR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port A Reset bit 12"]
    #[inline(always)]
    pub fn br12(&self) -> BR12_R {
        BR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port A Reset bit 13"]
    #[inline(always)]
    pub fn br13(&self) -> BR13_R {
        BR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port A Reset bit 14"]
    #[inline(always)]
    pub fn br14(&self) -> BR14_R {
        BR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port A Reset bit 15"]
    #[inline(always)]
    pub fn br15(&self) -> BR15_R {
        BR15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port A Reset bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn br0(&mut self) -> BR0_W<0> {
        BR0_W::new(self)
    }
    #[doc = "Bit 1 - Port A Reset bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn br1(&mut self) -> BR1_W<1> {
        BR1_W::new(self)
    }
    #[doc = "Bit 2 - Port A Reset bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn br2(&mut self) -> BR2_W<2> {
        BR2_W::new(self)
    }
    #[doc = "Bit 3 - Port A Reset bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn br3(&mut self) -> BR3_W<3> {
        BR3_W::new(self)
    }
    #[doc = "Bit 4 - Port A Reset bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn br4(&mut self) -> BR4_W<4> {
        BR4_W::new(self)
    }
    #[doc = "Bit 5 - Port A Reset bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn br5(&mut self) -> BR5_W<5> {
        BR5_W::new(self)
    }
    #[doc = "Bit 6 - Port A Reset bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn br6(&mut self) -> BR6_W<6> {
        BR6_W::new(self)
    }
    #[doc = "Bit 7 - Port A Reset bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn br7(&mut self) -> BR7_W<7> {
        BR7_W::new(self)
    }
    #[doc = "Bit 8 - Port A Reset bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn br8(&mut self) -> BR8_W<8> {
        BR8_W::new(self)
    }
    #[doc = "Bit 9 - Port A Reset bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn br9(&mut self) -> BR9_W<9> {
        BR9_W::new(self)
    }
    #[doc = "Bit 10 - Port A Reset bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn br10(&mut self) -> BR10_W<10> {
        BR10_W::new(self)
    }
    #[doc = "Bit 11 - Port A Reset bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn br11(&mut self) -> BR11_W<11> {
        BR11_W::new(self)
    }
    #[doc = "Bit 12 - Port A Reset bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn br12(&mut self) -> BR12_W<12> {
        BR12_W::new(self)
    }
    #[doc = "Bit 13 - Port A Reset bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn br13(&mut self) -> BR13_W<13> {
        BR13_W::new(self)
    }
    #[doc = "Bit 14 - Port A Reset bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn br14(&mut self) -> BR14_W<14> {
        BR14_W::new(self)
    }
    #[doc = "Bit 15 - Port A Reset bit 15"]
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
#[doc = "GPIO port bit reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brr](index.html) module"]
pub struct BRR_SPEC;
impl crate::RegisterSpec for BRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [brr::R](R) reader structure"]
impl crate::Readable for BRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [brr::W](W) writer structure"]
impl crate::Writable for BRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BRR to value 0"]
impl crate::Resettable for BRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
