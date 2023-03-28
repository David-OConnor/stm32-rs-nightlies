#[doc = "Register `EXTI_RPR1` reader"]
pub struct R(crate::R<EXTI_RPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_RPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_RPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_RPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_RPR1` writer"]
pub struct W(crate::W<EXTI_RPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_RPR1_SPEC>;
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
impl From<crate::W<EXTI_RPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_RPR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPIF0` reader - RPIF0"]
pub type RPIF0_R = crate::BitReader<bool>;
#[doc = "Field `RPIF0` writer - RPIF0"]
pub type RPIF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RPR1_SPEC, bool, O>;
#[doc = "Field `RPIF1` reader - RPIF1"]
pub type RPIF1_R = crate::BitReader<bool>;
#[doc = "Field `RPIF1` writer - RPIF1"]
pub type RPIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RPR1_SPEC, bool, O>;
#[doc = "Field `RPIF2` reader - RPIF2"]
pub type RPIF2_R = crate::BitReader<bool>;
#[doc = "Field `RPIF2` writer - RPIF2"]
pub type RPIF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RPR1_SPEC, bool, O>;
#[doc = "Field `RPIF3` reader - RPIF3"]
pub type RPIF3_R = crate::BitReader<bool>;
#[doc = "Field `RPIF3` writer - RPIF3"]
pub type RPIF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RPR1_SPEC, bool, O>;
#[doc = "Field `RPIF4` reader - RPIF4"]
pub type RPIF4_R = crate::BitReader<bool>;
#[doc = "Field `RPIF4` writer - RPIF4"]
pub type RPIF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RPR1_SPEC, bool, O>;
#[doc = "Field `RPIF5` reader - RPIF5"]
pub type RPIF5_R = crate::BitReader<bool>;
#[doc = "Field `RPIF5` writer - RPIF5"]
pub type RPIF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RPR1_SPEC, bool, O>;
#[doc = "Field `RPIF6` reader - RPIF6"]
pub type RPIF6_R = crate::BitReader<bool>;
#[doc = "Field `RPIF6` writer - RPIF6"]
pub type RPIF6_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RPR1_SPEC, bool, O>;
#[doc = "Field `RPIF7` reader - RPIF7"]
pub type RPIF7_R = crate::BitReader<bool>;
#[doc = "Field `RPIF7` writer - RPIF7"]
pub type RPIF7_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RPR1_SPEC, bool, O>;
#[doc = "Field `RPIF8` reader - RPIF8"]
pub type RPIF8_R = crate::BitReader<bool>;
#[doc = "Field `RPIF8` writer - RPIF8"]
pub type RPIF8_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RPR1_SPEC, bool, O>;
#[doc = "Field `RPIF9` reader - RPIF9"]
pub type RPIF9_R = crate::BitReader<bool>;
#[doc = "Field `RPIF9` writer - RPIF9"]
pub type RPIF9_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RPR1_SPEC, bool, O>;
#[doc = "Field `RPIF10` reader - RPIF10"]
pub type RPIF10_R = crate::BitReader<bool>;
#[doc = "Field `RPIF10` writer - RPIF10"]
pub type RPIF10_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RPR1_SPEC, bool, O>;
#[doc = "Field `RPIF11` reader - RPIF11"]
pub type RPIF11_R = crate::BitReader<bool>;
#[doc = "Field `RPIF11` writer - RPIF11"]
pub type RPIF11_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RPR1_SPEC, bool, O>;
#[doc = "Field `RPIF12` reader - RPIF12"]
pub type RPIF12_R = crate::BitReader<bool>;
#[doc = "Field `RPIF12` writer - RPIF12"]
pub type RPIF12_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RPR1_SPEC, bool, O>;
#[doc = "Field `RPIF13` reader - RPIF13"]
pub type RPIF13_R = crate::BitReader<bool>;
#[doc = "Field `RPIF13` writer - RPIF13"]
pub type RPIF13_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RPR1_SPEC, bool, O>;
#[doc = "Field `RPIF14` reader - RPIF14"]
pub type RPIF14_R = crate::BitReader<bool>;
#[doc = "Field `RPIF14` writer - RPIF14"]
pub type RPIF14_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RPR1_SPEC, bool, O>;
#[doc = "Field `RPIF15` reader - RPIF15"]
pub type RPIF15_R = crate::BitReader<bool>;
#[doc = "Field `RPIF15` writer - RPIF15"]
pub type RPIF15_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RPR1_SPEC, bool, O>;
#[doc = "Field `RPIF16` reader - RPIF16"]
pub type RPIF16_R = crate::BitReader<bool>;
#[doc = "Field `RPIF16` writer - RPIF16"]
pub type RPIF16_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_RPR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RPIF0"]
    #[inline(always)]
    pub fn rpif0(&self) -> RPIF0_R {
        RPIF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RPIF1"]
    #[inline(always)]
    pub fn rpif1(&self) -> RPIF1_R {
        RPIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RPIF2"]
    #[inline(always)]
    pub fn rpif2(&self) -> RPIF2_R {
        RPIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RPIF3"]
    #[inline(always)]
    pub fn rpif3(&self) -> RPIF3_R {
        RPIF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RPIF4"]
    #[inline(always)]
    pub fn rpif4(&self) -> RPIF4_R {
        RPIF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RPIF5"]
    #[inline(always)]
    pub fn rpif5(&self) -> RPIF5_R {
        RPIF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RPIF6"]
    #[inline(always)]
    pub fn rpif6(&self) -> RPIF6_R {
        RPIF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RPIF7"]
    #[inline(always)]
    pub fn rpif7(&self) -> RPIF7_R {
        RPIF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RPIF8"]
    #[inline(always)]
    pub fn rpif8(&self) -> RPIF8_R {
        RPIF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RPIF9"]
    #[inline(always)]
    pub fn rpif9(&self) -> RPIF9_R {
        RPIF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RPIF10"]
    #[inline(always)]
    pub fn rpif10(&self) -> RPIF10_R {
        RPIF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RPIF11"]
    #[inline(always)]
    pub fn rpif11(&self) -> RPIF11_R {
        RPIF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RPIF12"]
    #[inline(always)]
    pub fn rpif12(&self) -> RPIF12_R {
        RPIF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RPIF13"]
    #[inline(always)]
    pub fn rpif13(&self) -> RPIF13_R {
        RPIF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RPIF14"]
    #[inline(always)]
    pub fn rpif14(&self) -> RPIF14_R {
        RPIF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RPIF15"]
    #[inline(always)]
    pub fn rpif15(&self) -> RPIF15_R {
        RPIF15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RPIF16"]
    #[inline(always)]
    pub fn rpif16(&self) -> RPIF16_R {
        RPIF16_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RPIF0"]
    #[inline(always)]
    #[must_use]
    pub fn rpif0(&mut self) -> RPIF0_W<0> {
        RPIF0_W::new(self)
    }
    #[doc = "Bit 1 - RPIF1"]
    #[inline(always)]
    #[must_use]
    pub fn rpif1(&mut self) -> RPIF1_W<1> {
        RPIF1_W::new(self)
    }
    #[doc = "Bit 2 - RPIF2"]
    #[inline(always)]
    #[must_use]
    pub fn rpif2(&mut self) -> RPIF2_W<2> {
        RPIF2_W::new(self)
    }
    #[doc = "Bit 3 - RPIF3"]
    #[inline(always)]
    #[must_use]
    pub fn rpif3(&mut self) -> RPIF3_W<3> {
        RPIF3_W::new(self)
    }
    #[doc = "Bit 4 - RPIF4"]
    #[inline(always)]
    #[must_use]
    pub fn rpif4(&mut self) -> RPIF4_W<4> {
        RPIF4_W::new(self)
    }
    #[doc = "Bit 5 - RPIF5"]
    #[inline(always)]
    #[must_use]
    pub fn rpif5(&mut self) -> RPIF5_W<5> {
        RPIF5_W::new(self)
    }
    #[doc = "Bit 6 - RPIF6"]
    #[inline(always)]
    #[must_use]
    pub fn rpif6(&mut self) -> RPIF6_W<6> {
        RPIF6_W::new(self)
    }
    #[doc = "Bit 7 - RPIF7"]
    #[inline(always)]
    #[must_use]
    pub fn rpif7(&mut self) -> RPIF7_W<7> {
        RPIF7_W::new(self)
    }
    #[doc = "Bit 8 - RPIF8"]
    #[inline(always)]
    #[must_use]
    pub fn rpif8(&mut self) -> RPIF8_W<8> {
        RPIF8_W::new(self)
    }
    #[doc = "Bit 9 - RPIF9"]
    #[inline(always)]
    #[must_use]
    pub fn rpif9(&mut self) -> RPIF9_W<9> {
        RPIF9_W::new(self)
    }
    #[doc = "Bit 10 - RPIF10"]
    #[inline(always)]
    #[must_use]
    pub fn rpif10(&mut self) -> RPIF10_W<10> {
        RPIF10_W::new(self)
    }
    #[doc = "Bit 11 - RPIF11"]
    #[inline(always)]
    #[must_use]
    pub fn rpif11(&mut self) -> RPIF11_W<11> {
        RPIF11_W::new(self)
    }
    #[doc = "Bit 12 - RPIF12"]
    #[inline(always)]
    #[must_use]
    pub fn rpif12(&mut self) -> RPIF12_W<12> {
        RPIF12_W::new(self)
    }
    #[doc = "Bit 13 - RPIF13"]
    #[inline(always)]
    #[must_use]
    pub fn rpif13(&mut self) -> RPIF13_W<13> {
        RPIF13_W::new(self)
    }
    #[doc = "Bit 14 - RPIF14"]
    #[inline(always)]
    #[must_use]
    pub fn rpif14(&mut self) -> RPIF14_W<14> {
        RPIF14_W::new(self)
    }
    #[doc = "Bit 15 - RPIF15"]
    #[inline(always)]
    #[must_use]
    pub fn rpif15(&mut self) -> RPIF15_W<15> {
        RPIF15_W::new(self)
    }
    #[doc = "Bit 16 - RPIF16"]
    #[inline(always)]
    #[must_use]
    pub fn rpif16(&mut self) -> RPIF16_W<16> {
        RPIF16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains only register bits for configurable events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_rpr1](index.html) module"]
pub struct EXTI_RPR1_SPEC;
impl crate::RegisterSpec for EXTI_RPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_rpr1::R](R) reader structure"]
impl crate::Readable for EXTI_RPR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_rpr1::W](W) writer structure"]
impl crate::Writable for EXTI_RPR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTI_RPR1 to value 0"]
impl crate::Resettable for EXTI_RPR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
