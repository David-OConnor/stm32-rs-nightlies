#[doc = "Register `SPDIFRX_CR` reader"]
pub struct R(crate::R<SPDIFRX_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPDIFRX_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPDIFRX_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPDIFRX_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPDIFRX_CR` writer"]
pub struct W(crate::W<SPDIFRX_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPDIFRX_CR_SPEC>;
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
impl From<crate::W<SPDIFRX_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPDIFRX_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPDIFRXEN` reader - SPDIFRXEN"]
pub type SPDIFRXEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPDIFRXEN` writer - SPDIFRXEN"]
pub type SPDIFRXEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPDIFRX_CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `RXDMAEN` reader - RXDMAEN"]
pub type RXDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `RXDMAEN` writer - RXDMAEN"]
pub type RXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPDIFRX_CR_SPEC, bool, O>;
#[doc = "Field `RXSTEO` reader - RXSTEO"]
pub type RXSTEO_R = crate::BitReader<bool>;
#[doc = "Field `RXSTEO` writer - RXSTEO"]
pub type RXSTEO_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPDIFRX_CR_SPEC, bool, O>;
#[doc = "Field `DRFMT` reader - DRFMT"]
pub type DRFMT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRFMT` writer - DRFMT"]
pub type DRFMT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPDIFRX_CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PMSK` reader - PMSK"]
pub type PMSK_R = crate::BitReader<bool>;
#[doc = "Field `PMSK` writer - PMSK"]
pub type PMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPDIFRX_CR_SPEC, bool, O>;
#[doc = "Field `VMSK` reader - VMSK"]
pub type VMSK_R = crate::BitReader<bool>;
#[doc = "Field `VMSK` writer - VMSK"]
pub type VMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPDIFRX_CR_SPEC, bool, O>;
#[doc = "Field `CUMSK` reader - CUMSK"]
pub type CUMSK_R = crate::BitReader<bool>;
#[doc = "Field `CUMSK` writer - CUMSK"]
pub type CUMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPDIFRX_CR_SPEC, bool, O>;
#[doc = "Field `PTMSK` reader - PTMSK"]
pub type PTMSK_R = crate::BitReader<bool>;
#[doc = "Field `PTMSK` writer - PTMSK"]
pub type PTMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPDIFRX_CR_SPEC, bool, O>;
#[doc = "Field `CBDMAEN` reader - CBDMAEN"]
pub type CBDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `CBDMAEN` writer - CBDMAEN"]
pub type CBDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPDIFRX_CR_SPEC, bool, O>;
#[doc = "Field `CHSEL` reader - CHSEL"]
pub type CHSEL_R = crate::BitReader<bool>;
#[doc = "Field `CHSEL` writer - CHSEL"]
pub type CHSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPDIFRX_CR_SPEC, bool, O>;
#[doc = "Field `NBTR` reader - NBTR"]
pub type NBTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NBTR` writer - NBTR"]
pub type NBTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPDIFRX_CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `WFA` reader - WFA"]
pub type WFA_R = crate::BitReader<bool>;
#[doc = "Field `WFA` writer - WFA"]
pub type WFA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPDIFRX_CR_SPEC, bool, O>;
#[doc = "Field `INSEL` reader - INSEL"]
pub type INSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSEL` writer - INSEL"]
pub type INSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPDIFRX_CR_SPEC, u8, u8, 3, O>;
#[doc = "Field `CKSEN` reader - CKSEN"]
pub type CKSEN_R = crate::BitReader<bool>;
#[doc = "Field `CKSEN` writer - CKSEN"]
pub type CKSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPDIFRX_CR_SPEC, bool, O>;
#[doc = "Field `CKSBKPEN` reader - CKSBKPEN"]
pub type CKSBKPEN_R = crate::BitReader<bool>;
#[doc = "Field `CKSBKPEN` writer - CKSBKPEN"]
pub type CKSBKPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPDIFRX_CR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - SPDIFRXEN"]
    #[inline(always)]
    pub fn spdifrxen(&self) -> SPDIFRXEN_R {
        SPDIFRXEN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - RXDMAEN"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RXSTEO"]
    #[inline(always)]
    pub fn rxsteo(&self) -> RXSTEO_R {
        RXSTEO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - DRFMT"]
    #[inline(always)]
    pub fn drfmt(&self) -> DRFMT_R {
        DRFMT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - PMSK"]
    #[inline(always)]
    pub fn pmsk(&self) -> PMSK_R {
        PMSK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VMSK"]
    #[inline(always)]
    pub fn vmsk(&self) -> VMSK_R {
        VMSK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CUMSK"]
    #[inline(always)]
    pub fn cumsk(&self) -> CUMSK_R {
        CUMSK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PTMSK"]
    #[inline(always)]
    pub fn ptmsk(&self) -> PTMSK_R {
        PTMSK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CBDMAEN"]
    #[inline(always)]
    pub fn cbdmaen(&self) -> CBDMAEN_R {
        CBDMAEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CHSEL"]
    #[inline(always)]
    pub fn chsel(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - NBTR"]
    #[inline(always)]
    pub fn nbtr(&self) -> NBTR_R {
        NBTR_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - WFA"]
    #[inline(always)]
    pub fn wfa(&self) -> WFA_R {
        WFA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:18 - INSEL"]
    #[inline(always)]
    pub fn insel(&self) -> INSEL_R {
        INSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - CKSEN"]
    #[inline(always)]
    pub fn cksen(&self) -> CKSEN_R {
        CKSEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CKSBKPEN"]
    #[inline(always)]
    pub fn cksbkpen(&self) -> CKSBKPEN_R {
        CKSBKPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPDIFRXEN"]
    #[inline(always)]
    #[must_use]
    pub fn spdifrxen(&mut self) -> SPDIFRXEN_W<0> {
        SPDIFRXEN_W::new(self)
    }
    #[doc = "Bit 2 - RXDMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<2> {
        RXDMAEN_W::new(self)
    }
    #[doc = "Bit 3 - RXSTEO"]
    #[inline(always)]
    #[must_use]
    pub fn rxsteo(&mut self) -> RXSTEO_W<3> {
        RXSTEO_W::new(self)
    }
    #[doc = "Bits 4:5 - DRFMT"]
    #[inline(always)]
    #[must_use]
    pub fn drfmt(&mut self) -> DRFMT_W<4> {
        DRFMT_W::new(self)
    }
    #[doc = "Bit 6 - PMSK"]
    #[inline(always)]
    #[must_use]
    pub fn pmsk(&mut self) -> PMSK_W<6> {
        PMSK_W::new(self)
    }
    #[doc = "Bit 7 - VMSK"]
    #[inline(always)]
    #[must_use]
    pub fn vmsk(&mut self) -> VMSK_W<7> {
        VMSK_W::new(self)
    }
    #[doc = "Bit 8 - CUMSK"]
    #[inline(always)]
    #[must_use]
    pub fn cumsk(&mut self) -> CUMSK_W<8> {
        CUMSK_W::new(self)
    }
    #[doc = "Bit 9 - PTMSK"]
    #[inline(always)]
    #[must_use]
    pub fn ptmsk(&mut self) -> PTMSK_W<9> {
        PTMSK_W::new(self)
    }
    #[doc = "Bit 10 - CBDMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn cbdmaen(&mut self) -> CBDMAEN_W<10> {
        CBDMAEN_W::new(self)
    }
    #[doc = "Bit 11 - CHSEL"]
    #[inline(always)]
    #[must_use]
    pub fn chsel(&mut self) -> CHSEL_W<11> {
        CHSEL_W::new(self)
    }
    #[doc = "Bits 12:13 - NBTR"]
    #[inline(always)]
    #[must_use]
    pub fn nbtr(&mut self) -> NBTR_W<12> {
        NBTR_W::new(self)
    }
    #[doc = "Bit 14 - WFA"]
    #[inline(always)]
    #[must_use]
    pub fn wfa(&mut self) -> WFA_W<14> {
        WFA_W::new(self)
    }
    #[doc = "Bits 16:18 - INSEL"]
    #[inline(always)]
    #[must_use]
    pub fn insel(&mut self) -> INSEL_W<16> {
        INSEL_W::new(self)
    }
    #[doc = "Bit 20 - CKSEN"]
    #[inline(always)]
    #[must_use]
    pub fn cksen(&mut self) -> CKSEN_W<20> {
        CKSEN_W::new(self)
    }
    #[doc = "Bit 21 - CKSBKPEN"]
    #[inline(always)]
    #[must_use]
    pub fn cksbkpen(&mut self) -> CKSBKPEN_W<21> {
        CKSBKPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spdifrx_cr](index.html) module"]
pub struct SPDIFRX_CR_SPEC;
impl crate::RegisterSpec for SPDIFRX_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spdifrx_cr::R](R) reader structure"]
impl crate::Readable for SPDIFRX_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spdifrx_cr::W](W) writer structure"]
impl crate::Writable for SPDIFRX_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPDIFRX_CR to value 0"]
impl crate::Resettable for SPDIFRX_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
