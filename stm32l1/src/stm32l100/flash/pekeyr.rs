#[doc = "Register `PEKEYR` writer"]
pub struct W(crate::W<PEKEYR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEKEYR_SPEC>;
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
impl From<crate::W<PEKEYR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEKEYR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PEKEYR` writer - FLASH_PEC and data EEPROM key"]
pub type PEKEYR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PEKEYR_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - FLASH_PEC and data EEPROM key"]
    #[inline(always)]
    #[must_use]
    pub fn pekeyr(&mut self) -> PEKEYR_W<0> {
        PEKEYR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Program/erase key register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pekeyr](index.html) module"]
pub struct PEKEYR_SPEC;
impl crate::RegisterSpec for PEKEYR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pekeyr::W](W) writer structure"]
impl crate::Writable for PEKEYR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PEKEYR to value 0"]
impl crate::Resettable for PEKEYR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
