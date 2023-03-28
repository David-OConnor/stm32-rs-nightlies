#[doc = "Register `OTG_GUSBCFG` reader"]
pub struct R(crate::R<OTG_GUSBCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_GUSBCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_GUSBCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_GUSBCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_GUSBCFG` writer"]
pub struct W(crate::W<OTG_GUSBCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_GUSBCFG_SPEC>;
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
impl From<crate::W<OTG_GUSBCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_GUSBCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOCAL` reader - TOCAL"]
pub type TOCAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TOCAL` writer - TOCAL"]
pub type TOCAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTG_GUSBCFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `PHYSEL` reader - PHYSEL"]
pub type PHYSEL_R = crate::BitReader<bool>;
#[doc = "Field `PHYSEL` writer - PHYSEL"]
pub type PHYSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GUSBCFG_SPEC, bool, O>;
#[doc = "Field `SRPCAP` reader - SRPCAP"]
pub type SRPCAP_R = crate::BitReader<bool>;
#[doc = "Field `SRPCAP` writer - SRPCAP"]
pub type SRPCAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GUSBCFG_SPEC, bool, O>;
#[doc = "Field `HNPCAP` reader - HNPCAP"]
pub type HNPCAP_R = crate::BitReader<bool>;
#[doc = "Field `HNPCAP` writer - HNPCAP"]
pub type HNPCAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GUSBCFG_SPEC, bool, O>;
#[doc = "Field `TRDT` reader - TRDT"]
pub type TRDT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRDT` writer - TRDT"]
pub type TRDT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTG_GUSBCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `PHYLPC` reader - PHYLPC"]
pub type PHYLPC_R = crate::BitReader<bool>;
#[doc = "Field `PHYLPC` writer - PHYLPC"]
pub type PHYLPC_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GUSBCFG_SPEC, bool, O>;
#[doc = "Field `TSDPS` reader - TSDPS"]
pub type TSDPS_R = crate::BitReader<bool>;
#[doc = "Field `TSDPS` writer - TSDPS"]
pub type TSDPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GUSBCFG_SPEC, bool, O>;
#[doc = "Field `FHMOD` reader - FHMOD"]
pub type FHMOD_R = crate::BitReader<bool>;
#[doc = "Field `FHMOD` writer - FHMOD"]
pub type FHMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GUSBCFG_SPEC, bool, O>;
#[doc = "Field `FDMOD` reader - FDMOD"]
pub type FDMOD_R = crate::BitReader<bool>;
#[doc = "Field `FDMOD` writer - FDMOD"]
pub type FDMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GUSBCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - TOCAL"]
    #[inline(always)]
    pub fn tocal(&self) -> TOCAL_R {
        TOCAL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 6 - PHYSEL"]
    #[inline(always)]
    pub fn physel(&self) -> PHYSEL_R {
        PHYSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - SRPCAP"]
    #[inline(always)]
    pub fn srpcap(&self) -> SRPCAP_R {
        SRPCAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HNPCAP"]
    #[inline(always)]
    pub fn hnpcap(&self) -> HNPCAP_R {
        HNPCAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - TRDT"]
    #[inline(always)]
    pub fn trdt(&self) -> TRDT_R {
        TRDT_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - PHYLPC"]
    #[inline(always)]
    pub fn phylpc(&self) -> PHYLPC_R {
        PHYLPC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 22 - TSDPS"]
    #[inline(always)]
    pub fn tsdps(&self) -> TSDPS_R {
        TSDPS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 29 - FHMOD"]
    #[inline(always)]
    pub fn fhmod(&self) -> FHMOD_R {
        FHMOD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - FDMOD"]
    #[inline(always)]
    pub fn fdmod(&self) -> FDMOD_R {
        FDMOD_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - TOCAL"]
    #[inline(always)]
    #[must_use]
    pub fn tocal(&mut self) -> TOCAL_W<0> {
        TOCAL_W::new(self)
    }
    #[doc = "Bit 6 - PHYSEL"]
    #[inline(always)]
    #[must_use]
    pub fn physel(&mut self) -> PHYSEL_W<6> {
        PHYSEL_W::new(self)
    }
    #[doc = "Bit 8 - SRPCAP"]
    #[inline(always)]
    #[must_use]
    pub fn srpcap(&mut self) -> SRPCAP_W<8> {
        SRPCAP_W::new(self)
    }
    #[doc = "Bit 9 - HNPCAP"]
    #[inline(always)]
    #[must_use]
    pub fn hnpcap(&mut self) -> HNPCAP_W<9> {
        HNPCAP_W::new(self)
    }
    #[doc = "Bits 10:13 - TRDT"]
    #[inline(always)]
    #[must_use]
    pub fn trdt(&mut self) -> TRDT_W<10> {
        TRDT_W::new(self)
    }
    #[doc = "Bit 15 - PHYLPC"]
    #[inline(always)]
    #[must_use]
    pub fn phylpc(&mut self) -> PHYLPC_W<15> {
        PHYLPC_W::new(self)
    }
    #[doc = "Bit 22 - TSDPS"]
    #[inline(always)]
    #[must_use]
    pub fn tsdps(&mut self) -> TSDPS_W<22> {
        TSDPS_W::new(self)
    }
    #[doc = "Bit 29 - FHMOD"]
    #[inline(always)]
    #[must_use]
    pub fn fhmod(&mut self) -> FHMOD_W<29> {
        FHMOD_W::new(self)
    }
    #[doc = "Bit 30 - FDMOD"]
    #[inline(always)]
    #[must_use]
    pub fn fdmod(&mut self) -> FDMOD_W<30> {
        FDMOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register can be used to configure the core after power-on or a changing to host mode or device mode. It contains USB and USB-PHY related configuration parameters. The application must program this register before starting any transactions on either the AHB or the USB. Do not make changes to this register after the initial programming.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_gusbcfg](index.html) module"]
pub struct OTG_GUSBCFG_SPEC;
impl crate::RegisterSpec for OTG_GUSBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_gusbcfg::R](R) reader structure"]
impl crate::Readable for OTG_GUSBCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_gusbcfg::W](W) writer structure"]
impl crate::Writable for OTG_GUSBCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_GUSBCFG to value 0x1400"]
impl crate::Resettable for OTG_GUSBCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x1400;
}
