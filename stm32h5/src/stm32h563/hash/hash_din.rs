#[doc = "Register `HASH_DIN` writer"]
pub struct W(crate::W<HASH_DIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASH_DIN_SPEC>;
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
impl From<crate::W<HASH_DIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASH_DIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATAIN` writer - Data input Writing this register pushes the current register content into the FIFO, and the register takes the new value presented on the AHB bus. Reading this register returns zeros."]
pub type DATAIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HASH_DIN_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Data input Writing this register pushes the current register content into the FIFO, and the register takes the new value presented on the AHB bus. Reading this register returns zeros."]
    #[inline(always)]
    #[must_use]
    pub fn datain(&mut self) -> DATAIN_W<0> {
        DATAIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HASH data input register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_din](index.html) module"]
pub struct HASH_DIN_SPEC;
impl crate::RegisterSpec for HASH_DIN_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [hash_din::W](W) writer structure"]
impl crate::Writable for HASH_DIN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HASH_DIN to value 0"]
impl crate::Resettable for HASH_DIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
