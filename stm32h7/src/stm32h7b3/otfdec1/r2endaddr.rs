#[doc = "Register `R2ENDADDR` reader"]
pub struct R(crate::R<R2ENDADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R2ENDADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R2ENDADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R2ENDADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R2ENDADDR` writer"]
pub struct W(crate::W<R2ENDADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R2ENDADDR_SPEC>;
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
impl From<crate::W<R2ENDADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R2ENDADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGx_END_ADDR` reader - Region AXI end address"]
pub type REGX_END_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `REGx_END_ADDR` writer - Region AXI end address"]
pub type REGX_END_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, R2ENDADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Region AXI end address"]
    #[inline(always)]
    pub fn regx_end_addr(&self) -> REGX_END_ADDR_R {
        REGX_END_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region AXI end address"]
    #[inline(always)]
    #[must_use]
    pub fn regx_end_addr(&mut self) -> REGX_END_ADDR_W<0> {
        REGX_END_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTFDEC region x end address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r2endaddr](index.html) module"]
pub struct R2ENDADDR_SPEC;
impl crate::RegisterSpec for R2ENDADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r2endaddr::R](R) reader structure"]
impl crate::Readable for R2ENDADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r2endaddr::W](W) writer structure"]
impl crate::Writable for R2ENDADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets R2ENDADDR to value 0x0fff"]
impl crate::Resettable for R2ENDADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff;
}
