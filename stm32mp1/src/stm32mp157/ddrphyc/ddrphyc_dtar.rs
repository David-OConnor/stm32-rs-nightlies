#[doc = "Register `DDRPHYC_DTAR` reader"]
pub struct R(crate::R<DDRPHYC_DTAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DTAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DTAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DTAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_DTAR` writer"]
pub struct W(crate::W<DDRPHYC_DTAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DTAR_SPEC>;
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
impl From<crate::W<DDRPHYC_DTAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DTAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTCOL` reader - DTCOL"]
pub type DTCOL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DTCOL` writer - DTCOL"]
pub type DTCOL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DTAR_SPEC, u16, u16, 12, O>;
#[doc = "Field `DTROW` reader - DTROW"]
pub type DTROW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DTROW` writer - DTROW"]
pub type DTROW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DTAR_SPEC, u16, u16, 16, O>;
#[doc = "Field `DTBANK` reader - DTBANK"]
pub type DTBANK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTBANK` writer - DTBANK"]
pub type DTBANK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DTAR_SPEC, u8, u8, 3, O>;
#[doc = "Field `DTMPR` reader - DTMPR"]
pub type DTMPR_R = crate::BitReader<bool>;
#[doc = "Field `DTMPR` writer - DTMPR"]
pub type DTMPR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DTAR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:11 - DTCOL"]
    #[inline(always)]
    pub fn dtcol(&self) -> DTCOL_R {
        DTCOL_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:27 - DTROW"]
    #[inline(always)]
    pub fn dtrow(&self) -> DTROW_R {
        DTROW_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bits 28:30 - DTBANK"]
    #[inline(always)]
    pub fn dtbank(&self) -> DTBANK_R {
        DTBANK_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - DTMPR"]
    #[inline(always)]
    pub fn dtmpr(&self) -> DTMPR_R {
        DTMPR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - DTCOL"]
    #[inline(always)]
    #[must_use]
    pub fn dtcol(&mut self) -> DTCOL_W<0> {
        DTCOL_W::new(self)
    }
    #[doc = "Bits 12:27 - DTROW"]
    #[inline(always)]
    #[must_use]
    pub fn dtrow(&mut self) -> DTROW_W<12> {
        DTROW_W::new(self)
    }
    #[doc = "Bits 28:30 - DTBANK"]
    #[inline(always)]
    #[must_use]
    pub fn dtbank(&mut self) -> DTBANK_W<28> {
        DTBANK_W::new(self)
    }
    #[doc = "Bit 31 - DTMPR"]
    #[inline(always)]
    #[must_use]
    pub fn dtmpr(&mut self) -> DTMPR_W<31> {
        DTMPR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC DTA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dtar](index.html) module"]
pub struct DDRPHYC_DTAR_SPEC;
impl crate::RegisterSpec for DDRPHYC_DTAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_dtar::R](R) reader structure"]
impl crate::Readable for DDRPHYC_DTAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dtar::W](W) writer structure"]
impl crate::Writable for DDRPHYC_DTAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRPHYC_DTAR to value 0"]
impl crate::Resettable for DDRPHYC_DTAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
