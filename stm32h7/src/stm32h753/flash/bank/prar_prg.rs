#[doc = "Register `PRAR_PRG` reader"]
pub struct R(crate::R<PRAR_PRG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRAR_PRG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRAR_PRG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRAR_PRG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRAR_PRG` writer"]
pub struct W(crate::W<PRAR_PRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRAR_PRG_SPEC>;
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
impl From<crate::W<PRAR_PRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRAR_PRG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROT_AREA_START` reader - Bank 1 lowest PCROP protected address configuration"]
pub type PROT_AREA_START_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PROT_AREA_START` writer - Bank 1 lowest PCROP protected address configuration"]
pub type PROT_AREA_START_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRAR_PRG_SPEC, u16, u16, 12, O>;
#[doc = "Field `PROT_AREA_END` reader - Bank 1 highest PCROP protected address configuration"]
pub type PROT_AREA_END_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PROT_AREA_END` writer - Bank 1 highest PCROP protected address configuration"]
pub type PROT_AREA_END_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRAR_PRG_SPEC, u16, u16, 12, O>;
#[doc = "Field `DMEP` reader - Bank 1 PCROP protected erase enable option configuration bit"]
pub type DMEP_R = crate::BitReader<bool>;
#[doc = "Field `DMEP` writer - Bank 1 PCROP protected erase enable option configuration bit"]
pub type DMEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRAR_PRG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:11 - Bank 1 lowest PCROP protected address configuration"]
    #[inline(always)]
    pub fn prot_area_start(&self) -> PROT_AREA_START_R {
        PROT_AREA_START_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Bank 1 highest PCROP protected address configuration"]
    #[inline(always)]
    pub fn prot_area_end(&self) -> PROT_AREA_END_R {
        PROT_AREA_END_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - Bank 1 PCROP protected erase enable option configuration bit"]
    #[inline(always)]
    pub fn dmep(&self) -> DMEP_R {
        DMEP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Bank 1 lowest PCROP protected address configuration"]
    #[inline(always)]
    #[must_use]
    pub fn prot_area_start(&mut self) -> PROT_AREA_START_W<0> {
        PROT_AREA_START_W::new(self)
    }
    #[doc = "Bits 16:27 - Bank 1 highest PCROP protected address configuration"]
    #[inline(always)]
    #[must_use]
    pub fn prot_area_end(&mut self) -> PROT_AREA_END_W<16> {
        PROT_AREA_END_W::new(self)
    }
    #[doc = "Bit 31 - Bank 1 PCROP protected erase enable option configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn dmep(&mut self) -> DMEP_W<31> {
        DMEP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH protection address for bank 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prar_prg](index.html) module"]
pub struct PRAR_PRG_SPEC;
impl crate::RegisterSpec for PRAR_PRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prar_prg::R](R) reader structure"]
impl crate::Readable for PRAR_PRG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prar_prg::W](W) writer structure"]
impl crate::Writable for PRAR_PRG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRAR_PRG to value 0"]
impl crate::Resettable for PRAR_PRG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
