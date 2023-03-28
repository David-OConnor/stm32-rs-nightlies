#[doc = "Register `EXTI_SWIER1` reader"]
pub struct R(crate::R<EXTI_SWIER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_SWIER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_SWIER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_SWIER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_SWIER1` writer"]
pub struct W(crate::W<EXTI_SWIER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_SWIER1_SPEC>;
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
impl From<crate::W<EXTI_SWIER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_SWIER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWI0` reader - SWI0"]
pub type SWI0_R = crate::BitReader<bool>;
#[doc = "Field `SWI0` writer - SWI0"]
pub type SWI0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_SWIER1_SPEC, bool, O>;
#[doc = "Field `SWI1` reader - SWI1"]
pub type SWI1_R = crate::BitReader<bool>;
#[doc = "Field `SWI1` writer - SWI1"]
pub type SWI1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_SWIER1_SPEC, bool, O>;
#[doc = "Field `SWI2` reader - SWI2"]
pub type SWI2_R = crate::BitReader<bool>;
#[doc = "Field `SWI2` writer - SWI2"]
pub type SWI2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_SWIER1_SPEC, bool, O>;
#[doc = "Field `SWI3` reader - SWI3"]
pub type SWI3_R = crate::BitReader<bool>;
#[doc = "Field `SWI3` writer - SWI3"]
pub type SWI3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_SWIER1_SPEC, bool, O>;
#[doc = "Field `SWI4` reader - SWI4"]
pub type SWI4_R = crate::BitReader<bool>;
#[doc = "Field `SWI4` writer - SWI4"]
pub type SWI4_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_SWIER1_SPEC, bool, O>;
#[doc = "Field `SWI5` reader - SWI5"]
pub type SWI5_R = crate::BitReader<bool>;
#[doc = "Field `SWI5` writer - SWI5"]
pub type SWI5_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_SWIER1_SPEC, bool, O>;
#[doc = "Field `SWI6` reader - SWI6"]
pub type SWI6_R = crate::BitReader<bool>;
#[doc = "Field `SWI6` writer - SWI6"]
pub type SWI6_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_SWIER1_SPEC, bool, O>;
#[doc = "Field `SWI7` reader - SWI7"]
pub type SWI7_R = crate::BitReader<bool>;
#[doc = "Field `SWI7` writer - SWI7"]
pub type SWI7_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_SWIER1_SPEC, bool, O>;
#[doc = "Field `SWI8` reader - SWI8"]
pub type SWI8_R = crate::BitReader<bool>;
#[doc = "Field `SWI8` writer - SWI8"]
pub type SWI8_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_SWIER1_SPEC, bool, O>;
#[doc = "Field `SWI9` reader - SWI9"]
pub type SWI9_R = crate::BitReader<bool>;
#[doc = "Field `SWI9` writer - SWI9"]
pub type SWI9_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_SWIER1_SPEC, bool, O>;
#[doc = "Field `SWI10` reader - SWI10"]
pub type SWI10_R = crate::BitReader<bool>;
#[doc = "Field `SWI10` writer - SWI10"]
pub type SWI10_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_SWIER1_SPEC, bool, O>;
#[doc = "Field `SWI11` reader - SWI11"]
pub type SWI11_R = crate::BitReader<bool>;
#[doc = "Field `SWI11` writer - SWI11"]
pub type SWI11_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_SWIER1_SPEC, bool, O>;
#[doc = "Field `SWI12` reader - SWI12"]
pub type SWI12_R = crate::BitReader<bool>;
#[doc = "Field `SWI12` writer - SWI12"]
pub type SWI12_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_SWIER1_SPEC, bool, O>;
#[doc = "Field `SWI13` reader - SWI13"]
pub type SWI13_R = crate::BitReader<bool>;
#[doc = "Field `SWI13` writer - SWI13"]
pub type SWI13_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_SWIER1_SPEC, bool, O>;
#[doc = "Field `SWI14` reader - SWI14"]
pub type SWI14_R = crate::BitReader<bool>;
#[doc = "Field `SWI14` writer - SWI14"]
pub type SWI14_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_SWIER1_SPEC, bool, O>;
#[doc = "Field `SWI15` reader - SWI15"]
pub type SWI15_R = crate::BitReader<bool>;
#[doc = "Field `SWI15` writer - SWI15"]
pub type SWI15_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_SWIER1_SPEC, bool, O>;
#[doc = "Field `SWI16` reader - SWI16"]
pub type SWI16_R = crate::BitReader<bool>;
#[doc = "Field `SWI16` writer - SWI16"]
pub type SWI16_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_SWIER1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SWI0"]
    #[inline(always)]
    pub fn swi0(&self) -> SWI0_R {
        SWI0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SWI1"]
    #[inline(always)]
    pub fn swi1(&self) -> SWI1_R {
        SWI1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SWI2"]
    #[inline(always)]
    pub fn swi2(&self) -> SWI2_R {
        SWI2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SWI3"]
    #[inline(always)]
    pub fn swi3(&self) -> SWI3_R {
        SWI3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SWI4"]
    #[inline(always)]
    pub fn swi4(&self) -> SWI4_R {
        SWI4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SWI5"]
    #[inline(always)]
    pub fn swi5(&self) -> SWI5_R {
        SWI5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SWI6"]
    #[inline(always)]
    pub fn swi6(&self) -> SWI6_R {
        SWI6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SWI7"]
    #[inline(always)]
    pub fn swi7(&self) -> SWI7_R {
        SWI7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SWI8"]
    #[inline(always)]
    pub fn swi8(&self) -> SWI8_R {
        SWI8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SWI9"]
    #[inline(always)]
    pub fn swi9(&self) -> SWI9_R {
        SWI9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SWI10"]
    #[inline(always)]
    pub fn swi10(&self) -> SWI10_R {
        SWI10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SWI11"]
    #[inline(always)]
    pub fn swi11(&self) -> SWI11_R {
        SWI11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SWI12"]
    #[inline(always)]
    pub fn swi12(&self) -> SWI12_R {
        SWI12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SWI13"]
    #[inline(always)]
    pub fn swi13(&self) -> SWI13_R {
        SWI13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SWI14"]
    #[inline(always)]
    pub fn swi14(&self) -> SWI14_R {
        SWI14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SWI15"]
    #[inline(always)]
    pub fn swi15(&self) -> SWI15_R {
        SWI15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SWI16"]
    #[inline(always)]
    pub fn swi16(&self) -> SWI16_R {
        SWI16_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SWI0"]
    #[inline(always)]
    #[must_use]
    pub fn swi0(&mut self) -> SWI0_W<0> {
        SWI0_W::new(self)
    }
    #[doc = "Bit 1 - SWI1"]
    #[inline(always)]
    #[must_use]
    pub fn swi1(&mut self) -> SWI1_W<1> {
        SWI1_W::new(self)
    }
    #[doc = "Bit 2 - SWI2"]
    #[inline(always)]
    #[must_use]
    pub fn swi2(&mut self) -> SWI2_W<2> {
        SWI2_W::new(self)
    }
    #[doc = "Bit 3 - SWI3"]
    #[inline(always)]
    #[must_use]
    pub fn swi3(&mut self) -> SWI3_W<3> {
        SWI3_W::new(self)
    }
    #[doc = "Bit 4 - SWI4"]
    #[inline(always)]
    #[must_use]
    pub fn swi4(&mut self) -> SWI4_W<4> {
        SWI4_W::new(self)
    }
    #[doc = "Bit 5 - SWI5"]
    #[inline(always)]
    #[must_use]
    pub fn swi5(&mut self) -> SWI5_W<5> {
        SWI5_W::new(self)
    }
    #[doc = "Bit 6 - SWI6"]
    #[inline(always)]
    #[must_use]
    pub fn swi6(&mut self) -> SWI6_W<6> {
        SWI6_W::new(self)
    }
    #[doc = "Bit 7 - SWI7"]
    #[inline(always)]
    #[must_use]
    pub fn swi7(&mut self) -> SWI7_W<7> {
        SWI7_W::new(self)
    }
    #[doc = "Bit 8 - SWI8"]
    #[inline(always)]
    #[must_use]
    pub fn swi8(&mut self) -> SWI8_W<8> {
        SWI8_W::new(self)
    }
    #[doc = "Bit 9 - SWI9"]
    #[inline(always)]
    #[must_use]
    pub fn swi9(&mut self) -> SWI9_W<9> {
        SWI9_W::new(self)
    }
    #[doc = "Bit 10 - SWI10"]
    #[inline(always)]
    #[must_use]
    pub fn swi10(&mut self) -> SWI10_W<10> {
        SWI10_W::new(self)
    }
    #[doc = "Bit 11 - SWI11"]
    #[inline(always)]
    #[must_use]
    pub fn swi11(&mut self) -> SWI11_W<11> {
        SWI11_W::new(self)
    }
    #[doc = "Bit 12 - SWI12"]
    #[inline(always)]
    #[must_use]
    pub fn swi12(&mut self) -> SWI12_W<12> {
        SWI12_W::new(self)
    }
    #[doc = "Bit 13 - SWI13"]
    #[inline(always)]
    #[must_use]
    pub fn swi13(&mut self) -> SWI13_W<13> {
        SWI13_W::new(self)
    }
    #[doc = "Bit 14 - SWI14"]
    #[inline(always)]
    #[must_use]
    pub fn swi14(&mut self) -> SWI14_W<14> {
        SWI14_W::new(self)
    }
    #[doc = "Bit 15 - SWI15"]
    #[inline(always)]
    #[must_use]
    pub fn swi15(&mut self) -> SWI15_W<15> {
        SWI15_W::new(self)
    }
    #[doc = "Bit 16 - SWI16"]
    #[inline(always)]
    #[must_use]
    pub fn swi16(&mut self) -> SWI16_W<16> {
        SWI16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains only register bits for configurable events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_swier1](index.html) module"]
pub struct EXTI_SWIER1_SPEC;
impl crate::RegisterSpec for EXTI_SWIER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_swier1::R](R) reader structure"]
impl crate::Readable for EXTI_SWIER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_swier1::W](W) writer structure"]
impl crate::Writable for EXTI_SWIER1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTI_SWIER1 to value 0"]
impl crate::Resettable for EXTI_SWIER1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
