#[doc = "Register `OTFDEC_R2KEYR1` writer"]
pub struct W(crate::W<OTFDEC_R2KEYR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTFDEC_R2KEYR1_SPEC>;
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
impl From<crate::W<OTFDEC_R2KEYR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTFDEC_R2KEYR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGx_KEY` writer - Region key, bits \\[63:32\\]
Refer to the OTFDEC_RxKEYR0 register for description of the KEY\\[127:0\\]
bitfield."]
pub type REGX_KEY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OTFDEC_R2KEYR1_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Region key, bits \\[63:32\\]
Refer to the OTFDEC_RxKEYR0 register for description of the KEY\\[127:0\\]
bitfield."]
    #[inline(always)]
    #[must_use]
    pub fn regx_key(&mut self) -> REGX_KEY_W<0> {
        REGX_KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTFDEC region 2 key register 1\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otfdec_r2keyr1](index.html) module"]
pub struct OTFDEC_R2KEYR1_SPEC;
impl crate::RegisterSpec for OTFDEC_R2KEYR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [otfdec_r2keyr1::W](W) writer structure"]
impl crate::Writable for OTFDEC_R2KEYR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTFDEC_R2KEYR1 to value 0"]
impl crate::Resettable for OTFDEC_R2KEYR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
