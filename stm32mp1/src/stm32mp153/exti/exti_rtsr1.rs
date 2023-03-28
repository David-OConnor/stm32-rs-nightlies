#[doc = "Register `EXTI_RTSR1` reader"]
pub struct R(crate::R<EXTI_RTSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_RTSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_RTSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_RTSR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_RTSR1` writer"]
pub struct W(crate::W<EXTI_RTSR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_RTSR1_SPEC>;
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
impl From<crate::W<EXTI_RTSR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_RTSR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RT0` reader - RT0"]
pub type RT0_R = crate::BitReader<bool>;
#[doc = "Field `RT0` writer - RT0"]
pub type RT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RTSR1_SPEC, bool, O>;
#[doc = "Field `RT1` reader - RT1"]
pub type RT1_R = crate::BitReader<bool>;
#[doc = "Field `RT1` writer - RT1"]
pub type RT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RTSR1_SPEC, bool, O>;
#[doc = "Field `RT2` reader - RT2"]
pub type RT2_R = crate::BitReader<bool>;
#[doc = "Field `RT2` writer - RT2"]
pub type RT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RTSR1_SPEC, bool, O>;
#[doc = "Field `RT3` reader - RT3"]
pub type RT3_R = crate::BitReader<bool>;
#[doc = "Field `RT3` writer - RT3"]
pub type RT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RTSR1_SPEC, bool, O>;
#[doc = "Field `RT4` reader - RT4"]
pub type RT4_R = crate::BitReader<bool>;
#[doc = "Field `RT4` writer - RT4"]
pub type RT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RTSR1_SPEC, bool, O>;
#[doc = "Field `RT5` reader - RT5"]
pub type RT5_R = crate::BitReader<bool>;
#[doc = "Field `RT5` writer - RT5"]
pub type RT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RTSR1_SPEC, bool, O>;
#[doc = "Field `RT6` reader - RT6"]
pub type RT6_R = crate::BitReader<bool>;
#[doc = "Field `RT6` writer - RT6"]
pub type RT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RTSR1_SPEC, bool, O>;
#[doc = "Field `RT7` reader - RT7"]
pub type RT7_R = crate::BitReader<bool>;
#[doc = "Field `RT7` writer - RT7"]
pub type RT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RTSR1_SPEC, bool, O>;
#[doc = "Field `RT8` reader - RT8"]
pub type RT8_R = crate::BitReader<bool>;
#[doc = "Field `RT8` writer - RT8"]
pub type RT8_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RTSR1_SPEC, bool, O>;
#[doc = "Field `RT9` reader - RT9"]
pub type RT9_R = crate::BitReader<bool>;
#[doc = "Field `RT9` writer - RT9"]
pub type RT9_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RTSR1_SPEC, bool, O>;
#[doc = "Field `RT10` reader - RT10"]
pub type RT10_R = crate::BitReader<bool>;
#[doc = "Field `RT10` writer - RT10"]
pub type RT10_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RTSR1_SPEC, bool, O>;
#[doc = "Field `RT11` reader - RT11"]
pub type RT11_R = crate::BitReader<bool>;
#[doc = "Field `RT11` writer - RT11"]
pub type RT11_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RTSR1_SPEC, bool, O>;
#[doc = "Field `RT12` reader - RT12"]
pub type RT12_R = crate::BitReader<bool>;
#[doc = "Field `RT12` writer - RT12"]
pub type RT12_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RTSR1_SPEC, bool, O>;
#[doc = "Field `RT13` reader - RT13"]
pub type RT13_R = crate::BitReader<bool>;
#[doc = "Field `RT13` writer - RT13"]
pub type RT13_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RTSR1_SPEC, bool, O>;
#[doc = "Field `RT14` reader - RT14"]
pub type RT14_R = crate::BitReader<bool>;
#[doc = "Field `RT14` writer - RT14"]
pub type RT14_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RTSR1_SPEC, bool, O>;
#[doc = "Field `RT15` reader - RT15"]
pub type RT15_R = crate::BitReader<bool>;
#[doc = "Field `RT15` writer - RT15"]
pub type RT15_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RTSR1_SPEC, bool, O>;
#[doc = "Field `RT16` reader - RT16"]
pub type RT16_R = crate::BitReader<bool>;
#[doc = "Field `RT16` writer - RT16"]
pub type RT16_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RTSR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RT0"]
    #[inline(always)]
    pub fn rt0(&self) -> RT0_R {
        RT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RT1"]
    #[inline(always)]
    pub fn rt1(&self) -> RT1_R {
        RT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RT2"]
    #[inline(always)]
    pub fn rt2(&self) -> RT2_R {
        RT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RT3"]
    #[inline(always)]
    pub fn rt3(&self) -> RT3_R {
        RT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RT4"]
    #[inline(always)]
    pub fn rt4(&self) -> RT4_R {
        RT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RT5"]
    #[inline(always)]
    pub fn rt5(&self) -> RT5_R {
        RT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RT6"]
    #[inline(always)]
    pub fn rt6(&self) -> RT6_R {
        RT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RT7"]
    #[inline(always)]
    pub fn rt7(&self) -> RT7_R {
        RT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RT8"]
    #[inline(always)]
    pub fn rt8(&self) -> RT8_R {
        RT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RT9"]
    #[inline(always)]
    pub fn rt9(&self) -> RT9_R {
        RT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RT10"]
    #[inline(always)]
    pub fn rt10(&self) -> RT10_R {
        RT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RT11"]
    #[inline(always)]
    pub fn rt11(&self) -> RT11_R {
        RT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RT12"]
    #[inline(always)]
    pub fn rt12(&self) -> RT12_R {
        RT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RT13"]
    #[inline(always)]
    pub fn rt13(&self) -> RT13_R {
        RT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RT14"]
    #[inline(always)]
    pub fn rt14(&self) -> RT14_R {
        RT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RT15"]
    #[inline(always)]
    pub fn rt15(&self) -> RT15_R {
        RT15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RT16"]
    #[inline(always)]
    pub fn rt16(&self) -> RT16_R {
        RT16_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RT0"]
    #[inline(always)]
    #[must_use]
    pub fn rt0(&mut self) -> RT0_W<0> {
        RT0_W::new(self)
    }
    #[doc = "Bit 1 - RT1"]
    #[inline(always)]
    #[must_use]
    pub fn rt1(&mut self) -> RT1_W<1> {
        RT1_W::new(self)
    }
    #[doc = "Bit 2 - RT2"]
    #[inline(always)]
    #[must_use]
    pub fn rt2(&mut self) -> RT2_W<2> {
        RT2_W::new(self)
    }
    #[doc = "Bit 3 - RT3"]
    #[inline(always)]
    #[must_use]
    pub fn rt3(&mut self) -> RT3_W<3> {
        RT3_W::new(self)
    }
    #[doc = "Bit 4 - RT4"]
    #[inline(always)]
    #[must_use]
    pub fn rt4(&mut self) -> RT4_W<4> {
        RT4_W::new(self)
    }
    #[doc = "Bit 5 - RT5"]
    #[inline(always)]
    #[must_use]
    pub fn rt5(&mut self) -> RT5_W<5> {
        RT5_W::new(self)
    }
    #[doc = "Bit 6 - RT6"]
    #[inline(always)]
    #[must_use]
    pub fn rt6(&mut self) -> RT6_W<6> {
        RT6_W::new(self)
    }
    #[doc = "Bit 7 - RT7"]
    #[inline(always)]
    #[must_use]
    pub fn rt7(&mut self) -> RT7_W<7> {
        RT7_W::new(self)
    }
    #[doc = "Bit 8 - RT8"]
    #[inline(always)]
    #[must_use]
    pub fn rt8(&mut self) -> RT8_W<8> {
        RT8_W::new(self)
    }
    #[doc = "Bit 9 - RT9"]
    #[inline(always)]
    #[must_use]
    pub fn rt9(&mut self) -> RT9_W<9> {
        RT9_W::new(self)
    }
    #[doc = "Bit 10 - RT10"]
    #[inline(always)]
    #[must_use]
    pub fn rt10(&mut self) -> RT10_W<10> {
        RT10_W::new(self)
    }
    #[doc = "Bit 11 - RT11"]
    #[inline(always)]
    #[must_use]
    pub fn rt11(&mut self) -> RT11_W<11> {
        RT11_W::new(self)
    }
    #[doc = "Bit 12 - RT12"]
    #[inline(always)]
    #[must_use]
    pub fn rt12(&mut self) -> RT12_W<12> {
        RT12_W::new(self)
    }
    #[doc = "Bit 13 - RT13"]
    #[inline(always)]
    #[must_use]
    pub fn rt13(&mut self) -> RT13_W<13> {
        RT13_W::new(self)
    }
    #[doc = "Bit 14 - RT14"]
    #[inline(always)]
    #[must_use]
    pub fn rt14(&mut self) -> RT14_W<14> {
        RT14_W::new(self)
    }
    #[doc = "Bit 15 - RT15"]
    #[inline(always)]
    #[must_use]
    pub fn rt15(&mut self) -> RT15_W<15> {
        RT15_W::new(self)
    }
    #[doc = "Bit 16 - RT16"]
    #[inline(always)]
    #[must_use]
    pub fn rt16(&mut self) -> RT16_W<16> {
        RT16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains only register bits for configurable events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_rtsr1](index.html) module"]
pub struct EXTI_RTSR1_SPEC;
impl crate::RegisterSpec for EXTI_RTSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_rtsr1::R](R) reader structure"]
impl crate::Readable for EXTI_RTSR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_rtsr1::W](W) writer structure"]
impl crate::Writable for EXTI_RTSR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTI_RTSR1 to value 0"]
impl crate::Resettable for EXTI_RTSR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
