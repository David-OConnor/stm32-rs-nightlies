#[doc = "Register `ETZPC_TZMA0_SIZE` reader"]
pub struct R(crate::R<ETZPC_TZMA0_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETZPC_TZMA0_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETZPC_TZMA0_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETZPC_TZMA0_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETZPC_TZMA0_SIZE` writer"]
pub struct W(crate::W<ETZPC_TZMA0_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETZPC_TZMA0_SIZE_SPEC>;
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
impl From<crate::W<ETZPC_TZMA0_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETZPC_TZMA0_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `R0SIZE` reader - R0SIZE"]
pub type R0SIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `R0SIZE` writer - R0SIZE"]
pub type R0SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETZPC_TZMA0_SIZE_SPEC, u16, u16, 10, O>;
#[doc = "Field `LOCK` reader - LOCK"]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - LOCK"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETZPC_TZMA0_SIZE_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9 - R0SIZE"]
    #[inline(always)]
    pub fn r0size(&self) -> R0SIZE_R {
        R0SIZE_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - R0SIZE"]
    #[inline(always)]
    #[must_use]
    pub fn r0size(&mut self) -> R0SIZE_W<0> {
        R0SIZE_W::new(self)
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<31> {
        LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETZPC ROM secure size definition\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etzpc_tzma0_size](index.html) module"]
pub struct ETZPC_TZMA0_SIZE_SPEC;
impl crate::RegisterSpec for ETZPC_TZMA0_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etzpc_tzma0_size::R](R) reader structure"]
impl crate::Readable for ETZPC_TZMA0_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etzpc_tzma0_size::W](W) writer structure"]
impl crate::Writable for ETZPC_TZMA0_SIZE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETZPC_TZMA0_SIZE to value 0x03ff"]
impl crate::Resettable for ETZPC_TZMA0_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0x03ff;
}
