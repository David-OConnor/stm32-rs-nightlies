#[doc = "Register `FPR2` reader"]
pub struct R(crate::R<FPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPR2` writer"]
pub struct W(crate::W<FPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPR2_SPEC>;
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
impl From<crate::W<FPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPIF35` reader - FPIF35"]
pub type FPIF35_R = crate::BitReader<bool>;
#[doc = "Field `FPIF35` writer - FPIF35"]
pub type FPIF35_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPR2_SPEC, bool, O>;
#[doc = "Field `FPIF36` reader - FPIF36"]
pub type FPIF36_R = crate::BitReader<bool>;
#[doc = "Field `FPIF36` writer - FPIF36"]
pub type FPIF36_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPR2_SPEC, bool, O>;
#[doc = "Field `FPIF37` reader - FPIF37"]
pub type FPIF37_R = crate::BitReader<bool>;
#[doc = "Field `FPIF37` writer - FPIF37"]
pub type FPIF37_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPR2_SPEC, bool, O>;
#[doc = "Field `FPIF38` reader - FPIF38"]
pub type FPIF38_R = crate::BitReader<bool>;
#[doc = "Field `FPIF38` writer - FPIF38"]
pub type FPIF38_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - FPIF35"]
    #[inline(always)]
    pub fn fpif35(&self) -> FPIF35_R {
        FPIF35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FPIF36"]
    #[inline(always)]
    pub fn fpif36(&self) -> FPIF36_R {
        FPIF36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FPIF37"]
    #[inline(always)]
    pub fn fpif37(&self) -> FPIF37_R {
        FPIF37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FPIF38"]
    #[inline(always)]
    pub fn fpif38(&self) -> FPIF38_R {
        FPIF38_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - FPIF35"]
    #[inline(always)]
    #[must_use]
    pub fn fpif35(&mut self) -> FPIF35_W<3> {
        FPIF35_W::new(self)
    }
    #[doc = "Bit 4 - FPIF36"]
    #[inline(always)]
    #[must_use]
    pub fn fpif36(&mut self) -> FPIF36_W<4> {
        FPIF36_W::new(self)
    }
    #[doc = "Bit 5 - FPIF37"]
    #[inline(always)]
    #[must_use]
    pub fn fpif37(&mut self) -> FPIF37_W<5> {
        FPIF37_W::new(self)
    }
    #[doc = "Bit 6 - FPIF38"]
    #[inline(always)]
    #[must_use]
    pub fn fpif38(&mut self) -> FPIF38_W<6> {
        FPIF38_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI falling edge pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpr2](index.html) module"]
pub struct FPR2_SPEC;
impl crate::RegisterSpec for FPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fpr2::R](R) reader structure"]
impl crate::Readable for FPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpr2::W](W) writer structure"]
impl crate::Writable for FPR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FPR2 to value 0"]
impl crate::Resettable for FPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
