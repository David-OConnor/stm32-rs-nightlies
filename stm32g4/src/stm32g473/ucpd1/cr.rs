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
#[doc = "Field `TXMODE` reader - TXMODE"]
pub type TXMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXMODE` writer - TXMODE"]
pub type TXMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TXSEND` reader - TXSEND"]
pub type TXSEND_R = crate::BitReader<bool>;
#[doc = "Field `TXSEND` writer - TXSEND"]
pub type TXSEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TXHRST` reader - TXHRST"]
pub type TXHRST_R = crate::BitReader<bool>;
#[doc = "Field `TXHRST` writer - TXHRST"]
pub type TXHRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RXMODE` reader - RXMODE"]
pub type RXMODE_R = crate::BitReader<bool>;
#[doc = "Field `RXMODE` writer - RXMODE"]
pub type RXMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `PHYRXEN` reader - PHYRXEN"]
pub type PHYRXEN_R = crate::BitReader<bool>;
#[doc = "Field `PHYRXEN` writer - PHYRXEN"]
pub type PHYRXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `PHYCCSEL` reader - PHYCCSEL"]
pub type PHYCCSEL_R = crate::BitReader<bool>;
#[doc = "Field `PHYCCSEL` writer - PHYCCSEL"]
pub type PHYCCSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ANASUBMODE` reader - ANASUBMODE"]
pub type ANASUBMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ANASUBMODE` writer - ANASUBMODE"]
pub type ANASUBMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `ANAMODE` reader - ANAMODE"]
pub type ANAMODE_R = crate::BitReader<bool>;
#[doc = "Field `ANAMODE` writer - ANAMODE"]
pub type ANAMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CCENABLE` reader - CCENABLE"]
pub type CCENABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCENABLE` writer - CCENABLE"]
pub type CCENABLE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `FRSRXEN` reader - FRSRXEN"]
pub type FRSRXEN_R = crate::BitReader<bool>;
#[doc = "Field `FRSRXEN` writer - FRSRXEN"]
pub type FRSRXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `FRSTX` reader - FRSTX"]
pub type FRSTX_R = crate::BitReader<bool>;
#[doc = "Field `FRSTX` writer - FRSTX"]
pub type FRSTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RDCH` reader - RDCH"]
pub type RDCH_R = crate::BitReader<bool>;
#[doc = "Field `RDCH` writer - RDCH"]
pub type RDCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CC1TCDIS` reader - CC1TCDIS"]
pub type CC1TCDIS_R = crate::BitReader<bool>;
#[doc = "Field `CC1TCDIS` writer - CC1TCDIS"]
pub type CC1TCDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CC2TCDIS` reader - CC2TCDIS"]
pub type CC2TCDIS_R = crate::BitReader<bool>;
#[doc = "Field `CC2TCDIS` writer - CC2TCDIS"]
pub type CC2TCDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - TXMODE"]
    #[inline(always)]
    pub fn txmode(&self) -> TXMODE_R {
        TXMODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - TXSEND"]
    #[inline(always)]
    pub fn txsend(&self) -> TXSEND_R {
        TXSEND_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TXHRST"]
    #[inline(always)]
    pub fn txhrst(&self) -> TXHRST_R {
        TXHRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RXMODE"]
    #[inline(always)]
    pub fn rxmode(&self) -> RXMODE_R {
        RXMODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PHYRXEN"]
    #[inline(always)]
    pub fn phyrxen(&self) -> PHYRXEN_R {
        PHYRXEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PHYCCSEL"]
    #[inline(always)]
    pub fn phyccsel(&self) -> PHYCCSEL_R {
        PHYCCSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - ANASUBMODE"]
    #[inline(always)]
    pub fn anasubmode(&self) -> ANASUBMODE_R {
        ANASUBMODE_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - ANAMODE"]
    #[inline(always)]
    pub fn anamode(&self) -> ANAMODE_R {
        ANAMODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - CCENABLE"]
    #[inline(always)]
    pub fn ccenable(&self) -> CCENABLE_R {
        CCENABLE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 16 - FRSRXEN"]
    #[inline(always)]
    pub fn frsrxen(&self) -> FRSRXEN_R {
        FRSRXEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FRSTX"]
    #[inline(always)]
    pub fn frstx(&self) -> FRSTX_R {
        FRSTX_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RDCH"]
    #[inline(always)]
    pub fn rdch(&self) -> RDCH_R {
        RDCH_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - CC1TCDIS"]
    #[inline(always)]
    pub fn cc1tcdis(&self) -> CC1TCDIS_R {
        CC1TCDIS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CC2TCDIS"]
    #[inline(always)]
    pub fn cc2tcdis(&self) -> CC2TCDIS_R {
        CC2TCDIS_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - TXMODE"]
    #[inline(always)]
    #[must_use]
    pub fn txmode(&mut self) -> TXMODE_W<0> {
        TXMODE_W::new(self)
    }
    #[doc = "Bit 2 - TXSEND"]
    #[inline(always)]
    #[must_use]
    pub fn txsend(&mut self) -> TXSEND_W<2> {
        TXSEND_W::new(self)
    }
    #[doc = "Bit 3 - TXHRST"]
    #[inline(always)]
    #[must_use]
    pub fn txhrst(&mut self) -> TXHRST_W<3> {
        TXHRST_W::new(self)
    }
    #[doc = "Bit 4 - RXMODE"]
    #[inline(always)]
    #[must_use]
    pub fn rxmode(&mut self) -> RXMODE_W<4> {
        RXMODE_W::new(self)
    }
    #[doc = "Bit 5 - PHYRXEN"]
    #[inline(always)]
    #[must_use]
    pub fn phyrxen(&mut self) -> PHYRXEN_W<5> {
        PHYRXEN_W::new(self)
    }
    #[doc = "Bit 6 - PHYCCSEL"]
    #[inline(always)]
    #[must_use]
    pub fn phyccsel(&mut self) -> PHYCCSEL_W<6> {
        PHYCCSEL_W::new(self)
    }
    #[doc = "Bits 7:8 - ANASUBMODE"]
    #[inline(always)]
    #[must_use]
    pub fn anasubmode(&mut self) -> ANASUBMODE_W<7> {
        ANASUBMODE_W::new(self)
    }
    #[doc = "Bit 9 - ANAMODE"]
    #[inline(always)]
    #[must_use]
    pub fn anamode(&mut self) -> ANAMODE_W<9> {
        ANAMODE_W::new(self)
    }
    #[doc = "Bits 10:11 - CCENABLE"]
    #[inline(always)]
    #[must_use]
    pub fn ccenable(&mut self) -> CCENABLE_W<10> {
        CCENABLE_W::new(self)
    }
    #[doc = "Bit 16 - FRSRXEN"]
    #[inline(always)]
    #[must_use]
    pub fn frsrxen(&mut self) -> FRSRXEN_W<16> {
        FRSRXEN_W::new(self)
    }
    #[doc = "Bit 17 - FRSTX"]
    #[inline(always)]
    #[must_use]
    pub fn frstx(&mut self) -> FRSTX_W<17> {
        FRSTX_W::new(self)
    }
    #[doc = "Bit 18 - RDCH"]
    #[inline(always)]
    #[must_use]
    pub fn rdch(&mut self) -> RDCH_W<18> {
        RDCH_W::new(self)
    }
    #[doc = "Bit 20 - CC1TCDIS"]
    #[inline(always)]
    #[must_use]
    pub fn cc1tcdis(&mut self) -> CC1TCDIS_W<20> {
        CC1TCDIS_W::new(self)
    }
    #[doc = "Bit 21 - CC2TCDIS"]
    #[inline(always)]
    #[must_use]
    pub fn cc2tcdis(&mut self) -> CC2TCDIS_W<21> {
        CC2TCDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UCPD configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
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
