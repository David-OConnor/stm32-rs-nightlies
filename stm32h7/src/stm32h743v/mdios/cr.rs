#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Peripheral enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Peripheral enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `WRIE` reader - Register write interrupt enable"]
pub type WRIE_R = crate::BitReader<bool>;
#[doc = "Field `WRIE` writer - Register write interrupt enable"]
pub type WRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RDIE` reader - Register Read Interrupt Enable"]
pub type RDIE_R = crate::BitReader<bool>;
#[doc = "Field `RDIE` writer - Register Read Interrupt Enable"]
pub type RDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `EIE` reader - Error interrupt enable"]
pub type EIE_R = crate::BitReader<bool>;
#[doc = "Field `EIE` writer - Error interrupt enable"]
pub type EIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DPC` reader - Disable Preamble Check"]
pub type DPC_R = crate::BitReader<bool>;
#[doc = "Field `DPC` writer - Disable Preamble Check"]
pub type DPC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `PORT_ADDRESS` reader - Slaves's address"]
pub type PORT_ADDRESS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PORT_ADDRESS` writer - Slaves's address"]
pub type PORT_ADDRESS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Register write interrupt enable"]
    #[inline(always)]
    pub fn wrie(&self) -> WRIE_R {
        WRIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Register Read Interrupt Enable"]
    #[inline(always)]
    pub fn rdie(&self) -> RDIE_R {
        RDIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Error interrupt enable"]
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Disable Preamble Check"]
    #[inline(always)]
    pub fn dpc(&self) -> DPC_R {
        DPC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Slaves's address"]
    #[inline(always)]
    pub fn port_address(&self) -> PORT_ADDRESS_R {
        PORT_ADDRESS_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Register write interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn wrie(&mut self) -> WRIE_W<1> {
        WRIE_W::new(self)
    }
    #[doc = "Bit 2 - Register Read Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdie(&mut self) -> RDIE_W<2> {
        RDIE_W::new(self)
    }
    #[doc = "Bit 3 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eie(&mut self) -> EIE_W<3> {
        EIE_W::new(self)
    }
    #[doc = "Bit 7 - Disable Preamble Check"]
    #[inline(always)]
    #[must_use]
    pub fn dpc(&mut self) -> DPC_W<7> {
        DPC_W::new(self)
    }
    #[doc = "Bits 8:12 - Slaves's address"]
    #[inline(always)]
    #[must_use]
    pub fn port_address(&mut self) -> PORT_ADDRESS_W<8> {
        PORT_ADDRESS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDIOS configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
