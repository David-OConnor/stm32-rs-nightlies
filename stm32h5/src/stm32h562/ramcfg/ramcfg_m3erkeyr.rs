#[doc = "Register `RAMCFG_M3ERKEYR` writer"]
pub struct W(crate::W<RAMCFG_M3ERKEYR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAMCFG_M3ERKEYR_SPEC>;
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
impl From<crate::W<RAMCFG_M3ERKEYR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAMCFG_M3ERKEYR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERASEKEY` writer - Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the RAMCFG_MxCR register. 1) Write 0xCA into ERASEKEY\\[7:0\\]. 2) Write 0x53 into ERASEKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
pub type ERASEKEY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RAMCFG_M3ERKEYR_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the RAMCFG_MxCR register. 1) Write 0xCA into ERASEKEY\\[7:0\\]. 2) Write 0x53 into ERASEKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
    #[inline(always)]
    #[must_use]
    pub fn erasekey(&mut self) -> ERASEKEY_W<0> {
        ERASEKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAMCFG memory 3 erase key register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ramcfg_m3erkeyr](index.html) module"]
pub struct RAMCFG_M3ERKEYR_SPEC;
impl crate::RegisterSpec for RAMCFG_M3ERKEYR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ramcfg_m3erkeyr::W](W) writer structure"]
impl crate::Writable for RAMCFG_M3ERKEYR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RAMCFG_M3ERKEYR to value 0"]
impl crate::Resettable for RAMCFG_M3ERKEYR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
