#[doc = "Register `DDRPHYC_DX1DLLCR` reader"]
pub struct R(crate::R<DDRPHYC_DX1DLLCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DX1DLLCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DX1DLLCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DX1DLLCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_DX1DLLCR` writer"]
pub struct W(crate::W<DDRPHYC_DX1DLLCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DX1DLLCR_SPEC>;
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
impl From<crate::W<DDRPHYC_DX1DLLCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DX1DLLCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFBDLY` reader - SFBDLY"]
pub type SFBDLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SFBDLY` writer - SFBDLY"]
pub type SFBDLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRPHYC_DX1DLLCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `SFWDLY` reader - SFWDLY"]
pub type SFWDLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SFWDLY` writer - SFWDLY"]
pub type SFWDLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRPHYC_DX1DLLCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `MFBDLY` reader - MFBDLY"]
pub type MFBDLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MFBDLY` writer - MFBDLY"]
pub type MFBDLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRPHYC_DX1DLLCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `MFWDLY` reader - MFWDLY"]
pub type MFWDLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MFWDLY` writer - MFWDLY"]
pub type MFWDLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRPHYC_DX1DLLCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `SSTART` reader - SSTART"]
pub type SSTART_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SSTART` writer - SSTART"]
pub type SSTART_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRPHYC_DX1DLLCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `SDPHASE` reader - SDPHASE"]
pub type SDPHASE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDPHASE` writer - SDPHASE"]
pub type SDPHASE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRPHYC_DX1DLLCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `ATESTEN` reader - ATESTEN"]
pub type ATESTEN_R = crate::BitReader<bool>;
#[doc = "Field `ATESTEN` writer - ATESTEN"]
pub type ATESTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DX1DLLCR_SPEC, bool, O>;
#[doc = "Field `SDLBMODE` reader - SDLBMODE"]
pub type SDLBMODE_R = crate::BitReader<bool>;
#[doc = "Field `SDLBMODE` writer - SDLBMODE"]
pub type SDLBMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DX1DLLCR_SPEC, bool, O>;
#[doc = "Field `DLLSRST` reader - DLLSRST"]
pub type DLLSRST_R = crate::BitReader<bool>;
#[doc = "Field `DLLSRST` writer - DLLSRST"]
pub type DLLSRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DX1DLLCR_SPEC, bool, O>;
#[doc = "Field `DLLDIS` reader - DLLDIS"]
pub type DLLDIS_R = crate::BitReader<bool>;
#[doc = "Field `DLLDIS` writer - DLLDIS"]
pub type DLLDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DX1DLLCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - SFBDLY"]
    #[inline(always)]
    pub fn sfbdly(&self) -> SFBDLY_R {
        SFBDLY_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - SFWDLY"]
    #[inline(always)]
    pub fn sfwdly(&self) -> SFWDLY_R {
        SFWDLY_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - MFBDLY"]
    #[inline(always)]
    pub fn mfbdly(&self) -> MFBDLY_R {
        MFBDLY_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - MFWDLY"]
    #[inline(always)]
    pub fn mfwdly(&self) -> MFWDLY_R {
        MFWDLY_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:13 - SSTART"]
    #[inline(always)]
    pub fn sstart(&self) -> SSTART_R {
        SSTART_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:17 - SDPHASE"]
    #[inline(always)]
    pub fn sdphase(&self) -> SDPHASE_R {
        SDPHASE_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - ATESTEN"]
    #[inline(always)]
    pub fn atesten(&self) -> ATESTEN_R {
        ATESTEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SDLBMODE"]
    #[inline(always)]
    pub fn sdlbmode(&self) -> SDLBMODE_R {
        SDLBMODE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 30 - DLLSRST"]
    #[inline(always)]
    pub fn dllsrst(&self) -> DLLSRST_R {
        DLLSRST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DLLDIS"]
    #[inline(always)]
    pub fn dlldis(&self) -> DLLDIS_R {
        DLLDIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SFBDLY"]
    #[inline(always)]
    #[must_use]
    pub fn sfbdly(&mut self) -> SFBDLY_W<0> {
        SFBDLY_W::new(self)
    }
    #[doc = "Bits 3:5 - SFWDLY"]
    #[inline(always)]
    #[must_use]
    pub fn sfwdly(&mut self) -> SFWDLY_W<3> {
        SFWDLY_W::new(self)
    }
    #[doc = "Bits 6:8 - MFBDLY"]
    #[inline(always)]
    #[must_use]
    pub fn mfbdly(&mut self) -> MFBDLY_W<6> {
        MFBDLY_W::new(self)
    }
    #[doc = "Bits 9:11 - MFWDLY"]
    #[inline(always)]
    #[must_use]
    pub fn mfwdly(&mut self) -> MFWDLY_W<9> {
        MFWDLY_W::new(self)
    }
    #[doc = "Bits 12:13 - SSTART"]
    #[inline(always)]
    #[must_use]
    pub fn sstart(&mut self) -> SSTART_W<12> {
        SSTART_W::new(self)
    }
    #[doc = "Bits 14:17 - SDPHASE"]
    #[inline(always)]
    #[must_use]
    pub fn sdphase(&mut self) -> SDPHASE_W<14> {
        SDPHASE_W::new(self)
    }
    #[doc = "Bit 18 - ATESTEN"]
    #[inline(always)]
    #[must_use]
    pub fn atesten(&mut self) -> ATESTEN_W<18> {
        ATESTEN_W::new(self)
    }
    #[doc = "Bit 19 - SDLBMODE"]
    #[inline(always)]
    #[must_use]
    pub fn sdlbmode(&mut self) -> SDLBMODE_W<19> {
        SDLBMODE_W::new(self)
    }
    #[doc = "Bit 30 - DLLSRST"]
    #[inline(always)]
    #[must_use]
    pub fn dllsrst(&mut self) -> DLLSRST_W<30> {
        DLLSRST_W::new(self)
    }
    #[doc = "Bit 31 - DLLDIS"]
    #[inline(always)]
    #[must_use]
    pub fn dlldis(&mut self) -> DLLDIS_W<31> {
        DLLDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC byte lane 1 DLLC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx1dllcr](index.html) module"]
pub struct DDRPHYC_DX1DLLCR_SPEC;
impl crate::RegisterSpec for DDRPHYC_DX1DLLCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_dx1dllcr::R](R) reader structure"]
impl crate::Readable for DDRPHYC_DX1DLLCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx1dllcr::W](W) writer structure"]
impl crate::Writable for DDRPHYC_DX1DLLCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRPHYC_DX1DLLCR to value 0x4000_0000"]
impl crate::Resettable for DDRPHYC_DX1DLLCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000_0000;
}
