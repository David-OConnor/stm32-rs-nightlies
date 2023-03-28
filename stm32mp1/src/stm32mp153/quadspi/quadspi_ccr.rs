#[doc = "Register `QUADSPI_CCR` reader"]
pub struct R(crate::R<QUADSPI_CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QUADSPI_CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QUADSPI_CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QUADSPI_CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QUADSPI_CCR` writer"]
pub struct W(crate::W<QUADSPI_CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QUADSPI_CCR_SPEC>;
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
impl From<crate::W<QUADSPI_CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QUADSPI_CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INSTRUCTION` reader - INSTRUCTION"]
pub type INSTRUCTION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSTRUCTION` writer - INSTRUCTION"]
pub type INSTRUCTION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, QUADSPI_CCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `IMODE` reader - IMODE"]
pub type IMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IMODE` writer - IMODE"]
pub type IMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, QUADSPI_CCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `ADMODE` reader - ADMODE"]
pub type ADMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADMODE` writer - ADMODE"]
pub type ADMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, QUADSPI_CCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `ADSIZE` reader - ADSIZE"]
pub type ADSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADSIZE` writer - ADSIZE"]
pub type ADSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, QUADSPI_CCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `ABMODE` reader - ABMODE"]
pub type ABMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ABMODE` writer - ABMODE"]
pub type ABMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, QUADSPI_CCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `ABSIZE` reader - ABSIZE"]
pub type ABSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ABSIZE` writer - ABSIZE"]
pub type ABSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, QUADSPI_CCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DCYC` reader - DCYC"]
pub type DCYC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCYC` writer - DCYC"]
pub type DCYC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, QUADSPI_CCR_SPEC, u8, u8, 5, O>;
#[doc = "Field `DMODE` reader - DMODE"]
pub type DMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMODE` writer - DMODE"]
pub type DMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, QUADSPI_CCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `FMODE` reader - FMODE"]
pub type FMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FMODE` writer - FMODE"]
pub type FMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, QUADSPI_CCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `SIOO` reader - SIOO"]
pub type SIOO_R = crate::BitReader<bool>;
#[doc = "Field `SIOO` writer - SIOO"]
pub type SIOO_W<'a, const O: u8> = crate::BitWriter<'a, u32, QUADSPI_CCR_SPEC, bool, O>;
#[doc = "Field `FRCM` reader - FRCM"]
pub type FRCM_R = crate::BitReader<bool>;
#[doc = "Field `FRCM` writer - FRCM"]
pub type FRCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, QUADSPI_CCR_SPEC, bool, O>;
#[doc = "Field `DHHC` reader - DHHC"]
pub type DHHC_R = crate::BitReader<bool>;
#[doc = "Field `DHHC` writer - DHHC"]
pub type DHHC_W<'a, const O: u8> = crate::BitWriter<'a, u32, QUADSPI_CCR_SPEC, bool, O>;
#[doc = "Field `DDRM` reader - DDRM"]
pub type DDRM_R = crate::BitReader<bool>;
#[doc = "Field `DDRM` writer - DDRM"]
pub type DDRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, QUADSPI_CCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - INSTRUCTION"]
    #[inline(always)]
    pub fn instruction(&self) -> INSTRUCTION_R {
        INSTRUCTION_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - IMODE"]
    #[inline(always)]
    pub fn imode(&self) -> IMODE_R {
        IMODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - ADMODE"]
    #[inline(always)]
    pub fn admode(&self) -> ADMODE_R {
        ADMODE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - ADSIZE"]
    #[inline(always)]
    pub fn adsize(&self) -> ADSIZE_R {
        ADSIZE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - ABMODE"]
    #[inline(always)]
    pub fn abmode(&self) -> ABMODE_R {
        ABMODE_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - ABSIZE"]
    #[inline(always)]
    pub fn absize(&self) -> ABSIZE_R {
        ABSIZE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:22 - DCYC"]
    #[inline(always)]
    pub fn dcyc(&self) -> DCYC_R {
        DCYC_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 24:25 - DMODE"]
    #[inline(always)]
    pub fn dmode(&self) -> DMODE_R {
        DMODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - FMODE"]
    #[inline(always)]
    pub fn fmode(&self) -> FMODE_R {
        FMODE_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - SIOO"]
    #[inline(always)]
    pub fn sioo(&self) -> SIOO_R {
        SIOO_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - FRCM"]
    #[inline(always)]
    pub fn frcm(&self) -> FRCM_R {
        FRCM_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DHHC"]
    #[inline(always)]
    pub fn dhhc(&self) -> DHHC_R {
        DHHC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DDRM"]
    #[inline(always)]
    pub fn ddrm(&self) -> DDRM_R {
        DDRM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - INSTRUCTION"]
    #[inline(always)]
    #[must_use]
    pub fn instruction(&mut self) -> INSTRUCTION_W<0> {
        INSTRUCTION_W::new(self)
    }
    #[doc = "Bits 8:9 - IMODE"]
    #[inline(always)]
    #[must_use]
    pub fn imode(&mut self) -> IMODE_W<8> {
        IMODE_W::new(self)
    }
    #[doc = "Bits 10:11 - ADMODE"]
    #[inline(always)]
    #[must_use]
    pub fn admode(&mut self) -> ADMODE_W<10> {
        ADMODE_W::new(self)
    }
    #[doc = "Bits 12:13 - ADSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn adsize(&mut self) -> ADSIZE_W<12> {
        ADSIZE_W::new(self)
    }
    #[doc = "Bits 14:15 - ABMODE"]
    #[inline(always)]
    #[must_use]
    pub fn abmode(&mut self) -> ABMODE_W<14> {
        ABMODE_W::new(self)
    }
    #[doc = "Bits 16:17 - ABSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn absize(&mut self) -> ABSIZE_W<16> {
        ABSIZE_W::new(self)
    }
    #[doc = "Bits 18:22 - DCYC"]
    #[inline(always)]
    #[must_use]
    pub fn dcyc(&mut self) -> DCYC_W<18> {
        DCYC_W::new(self)
    }
    #[doc = "Bits 24:25 - DMODE"]
    #[inline(always)]
    #[must_use]
    pub fn dmode(&mut self) -> DMODE_W<24> {
        DMODE_W::new(self)
    }
    #[doc = "Bits 26:27 - FMODE"]
    #[inline(always)]
    #[must_use]
    pub fn fmode(&mut self) -> FMODE_W<26> {
        FMODE_W::new(self)
    }
    #[doc = "Bit 28 - SIOO"]
    #[inline(always)]
    #[must_use]
    pub fn sioo(&mut self) -> SIOO_W<28> {
        SIOO_W::new(self)
    }
    #[doc = "Bit 29 - FRCM"]
    #[inline(always)]
    #[must_use]
    pub fn frcm(&mut self) -> FRCM_W<29> {
        FRCM_W::new(self)
    }
    #[doc = "Bit 30 - DHHC"]
    #[inline(always)]
    #[must_use]
    pub fn dhhc(&mut self) -> DHHC_W<30> {
        DHHC_W::new(self)
    }
    #[doc = "Bit 31 - DDRM"]
    #[inline(always)]
    #[must_use]
    pub fn ddrm(&mut self) -> DDRM_W<31> {
        DDRM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QUADSPI communication configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quadspi_ccr](index.html) module"]
pub struct QUADSPI_CCR_SPEC;
impl crate::RegisterSpec for QUADSPI_CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [quadspi_ccr::R](R) reader structure"]
impl crate::Readable for QUADSPI_CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [quadspi_ccr::W](W) writer structure"]
impl crate::Writable for QUADSPI_CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets QUADSPI_CCR to value 0"]
impl crate::Resettable for QUADSPI_CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
