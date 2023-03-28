#[doc = "Register `TTTMK` reader"]
pub struct R(crate::R<TTTMK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTTMK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTTMK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTTMK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TTTMK` writer"]
pub struct W(crate::W<TTTMK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TTTMK_SPEC>;
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
impl From<crate::W<TTTMK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TTTMK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TM` reader - Time Mark"]
pub type TM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TM` writer - Time Mark"]
pub type TM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTTMK_SPEC, u16, u16, 16, O>;
#[doc = "Field `TICC` reader - Time Mark Cycle Code"]
pub type TICC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TICC` writer - Time Mark Cycle Code"]
pub type TICC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTTMK_SPEC, u8, u8, 7, O>;
#[doc = "Field `LCKM` reader - TT Time Mark Register Locked"]
pub type LCKM_R = crate::BitReader<bool>;
#[doc = "Field `LCKM` writer - TT Time Mark Register Locked"]
pub type LCKM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTTMK_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - Time Mark"]
    #[inline(always)]
    pub fn tm(&self) -> TM_R {
        TM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:22 - Time Mark Cycle Code"]
    #[inline(always)]
    pub fn ticc(&self) -> TICC_R {
        TICC_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - TT Time Mark Register Locked"]
    #[inline(always)]
    pub fn lckm(&self) -> LCKM_R {
        LCKM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Time Mark"]
    #[inline(always)]
    #[must_use]
    pub fn tm(&mut self) -> TM_W<0> {
        TM_W::new(self)
    }
    #[doc = "Bits 16:22 - Time Mark Cycle Code"]
    #[inline(always)]
    #[must_use]
    pub fn ticc(&mut self) -> TICC_W<16> {
        TICC_W::new(self)
    }
    #[doc = "Bit 31 - TT Time Mark Register Locked"]
    #[inline(always)]
    #[must_use]
    pub fn lckm(&mut self) -> LCKM_W<31> {
        LCKM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN TT Time Mark Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tttmk](index.html) module"]
pub struct TTTMK_SPEC;
impl crate::RegisterSpec for TTTMK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tttmk::R](R) reader structure"]
impl crate::Readable for TTTMK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tttmk::W](W) writer structure"]
impl crate::Writable for TTTMK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TTTMK to value 0"]
impl crate::Resettable for TTTMK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
