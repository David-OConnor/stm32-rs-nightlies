#[doc = "Register `FDCAN_TTIE` reader"]
pub struct R(crate::R<FDCAN_TTIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TTIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TTIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TTIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TTIE` writer"]
pub struct W(crate::W<FDCAN_TTIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TTIE_SPEC>;
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
impl From<crate::W<FDCAN_TTIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TTIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SBCE` reader - SBCE"]
pub type SBCE_R = crate::BitReader<bool>;
#[doc = "Field `SBCE` writer - SBCE"]
pub type SBCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIE_SPEC, bool, O>;
#[doc = "Field `SMCE` reader - SMCE"]
pub type SMCE_R = crate::BitReader<bool>;
#[doc = "Field `SMCE` writer - SMCE"]
pub type SMCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIE_SPEC, bool, O>;
#[doc = "Field `CSME` reader - CSME"]
pub type CSME_R = crate::BitReader<bool>;
#[doc = "Field `CSME` writer - CSME"]
pub type CSME_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIE_SPEC, bool, O>;
#[doc = "Field `SOGE` reader - SOGE"]
pub type SOGE_R = crate::BitReader<bool>;
#[doc = "Field `SOGE` writer - SOGE"]
pub type SOGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIE_SPEC, bool, O>;
#[doc = "Field `RTMIE` reader - RTMIE"]
pub type RTMIE_R = crate::BitReader<bool>;
#[doc = "Field `RTMIE` writer - RTMIE"]
pub type RTMIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIE_SPEC, bool, O>;
#[doc = "Field `TTMIE` reader - TTMIE"]
pub type TTMIE_R = crate::BitReader<bool>;
#[doc = "Field `TTMIE` writer - TTMIE"]
pub type TTMIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIE_SPEC, bool, O>;
#[doc = "Field `SWEE` reader - SWEE"]
pub type SWEE_R = crate::BitReader<bool>;
#[doc = "Field `SWEE` writer - SWEE"]
pub type SWEE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIE_SPEC, bool, O>;
#[doc = "Field `GTWE` reader - GTWE"]
pub type GTWE_R = crate::BitReader<bool>;
#[doc = "Field `GTWE` writer - GTWE"]
pub type GTWE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIE_SPEC, bool, O>;
#[doc = "Field `GTDE` reader - GTDE"]
pub type GTDE_R = crate::BitReader<bool>;
#[doc = "Field `GTDE` writer - GTDE"]
pub type GTDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIE_SPEC, bool, O>;
#[doc = "Field `GTEE` reader - GTEE"]
pub type GTEE_R = crate::BitReader<bool>;
#[doc = "Field `GTEE` writer - GTEE"]
pub type GTEE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIE_SPEC, bool, O>;
#[doc = "Field `TXUE` reader - TXUE"]
pub type TXUE_R = crate::BitReader<bool>;
#[doc = "Field `TXUE` writer - TXUE"]
pub type TXUE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIE_SPEC, bool, O>;
#[doc = "Field `TXOE` reader - TXOE"]
pub type TXOE_R = crate::BitReader<bool>;
#[doc = "Field `TXOE` writer - TXOE"]
pub type TXOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIE_SPEC, bool, O>;
#[doc = "Field `SE1E` reader - SE1E"]
pub type SE1E_R = crate::BitReader<bool>;
#[doc = "Field `SE1E` writer - SE1E"]
pub type SE1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIE_SPEC, bool, O>;
#[doc = "Field `SE2E` reader - SE2E"]
pub type SE2E_R = crate::BitReader<bool>;
#[doc = "Field `SE2E` writer - SE2E"]
pub type SE2E_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIE_SPEC, bool, O>;
#[doc = "Field `ELCE` reader - ELCE"]
pub type ELCE_R = crate::BitReader<bool>;
#[doc = "Field `ELCE` writer - ELCE"]
pub type ELCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIE_SPEC, bool, O>;
#[doc = "Field `IWTE` reader - IWTE"]
pub type IWTE_R = crate::BitReader<bool>;
#[doc = "Field `IWTE` writer - IWTE"]
pub type IWTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIE_SPEC, bool, O>;
#[doc = "Field `WTE` reader - WTE"]
pub type WTE_R = crate::BitReader<bool>;
#[doc = "Field `WTE` writer - WTE"]
pub type WTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIE_SPEC, bool, O>;
#[doc = "Field `AWE` reader - AWE"]
pub type AWE_R = crate::BitReader<bool>;
#[doc = "Field `AWE` writer - AWE"]
pub type AWE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIE_SPEC, bool, O>;
#[doc = "Field `CERE` reader - CERE"]
pub type CERE_R = crate::BitReader<bool>;
#[doc = "Field `CERE` writer - CERE"]
pub type CERE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTIE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SBCE"]
    #[inline(always)]
    pub fn sbce(&self) -> SBCE_R {
        SBCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SMCE"]
    #[inline(always)]
    pub fn smce(&self) -> SMCE_R {
        SMCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CSME"]
    #[inline(always)]
    pub fn csme(&self) -> CSME_R {
        CSME_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SOGE"]
    #[inline(always)]
    pub fn soge(&self) -> SOGE_R {
        SOGE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTMIE"]
    #[inline(always)]
    pub fn rtmie(&self) -> RTMIE_R {
        RTMIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TTMIE"]
    #[inline(always)]
    pub fn ttmie(&self) -> TTMIE_R {
        TTMIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SWEE"]
    #[inline(always)]
    pub fn swee(&self) -> SWEE_R {
        SWEE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GTWE"]
    #[inline(always)]
    pub fn gtwe(&self) -> GTWE_R {
        GTWE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GTDE"]
    #[inline(always)]
    pub fn gtde(&self) -> GTDE_R {
        GTDE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GTEE"]
    #[inline(always)]
    pub fn gtee(&self) -> GTEE_R {
        GTEE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TXUE"]
    #[inline(always)]
    pub fn txue(&self) -> TXUE_R {
        TXUE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TXOE"]
    #[inline(always)]
    pub fn txoe(&self) -> TXOE_R {
        TXOE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SE1E"]
    #[inline(always)]
    pub fn se1e(&self) -> SE1E_R {
        SE1E_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SE2E"]
    #[inline(always)]
    pub fn se2e(&self) -> SE2E_R {
        SE2E_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ELCE"]
    #[inline(always)]
    pub fn elce(&self) -> ELCE_R {
        ELCE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - IWTE"]
    #[inline(always)]
    pub fn iwte(&self) -> IWTE_R {
        IWTE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - WTE"]
    #[inline(always)]
    pub fn wte(&self) -> WTE_R {
        WTE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AWE"]
    #[inline(always)]
    pub fn awe(&self) -> AWE_R {
        AWE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CERE"]
    #[inline(always)]
    pub fn cere(&self) -> CERE_R {
        CERE_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SBCE"]
    #[inline(always)]
    #[must_use]
    pub fn sbce(&mut self) -> SBCE_W<0> {
        SBCE_W::new(self)
    }
    #[doc = "Bit 1 - SMCE"]
    #[inline(always)]
    #[must_use]
    pub fn smce(&mut self) -> SMCE_W<1> {
        SMCE_W::new(self)
    }
    #[doc = "Bit 2 - CSME"]
    #[inline(always)]
    #[must_use]
    pub fn csme(&mut self) -> CSME_W<2> {
        CSME_W::new(self)
    }
    #[doc = "Bit 3 - SOGE"]
    #[inline(always)]
    #[must_use]
    pub fn soge(&mut self) -> SOGE_W<3> {
        SOGE_W::new(self)
    }
    #[doc = "Bit 4 - RTMIE"]
    #[inline(always)]
    #[must_use]
    pub fn rtmie(&mut self) -> RTMIE_W<4> {
        RTMIE_W::new(self)
    }
    #[doc = "Bit 5 - TTMIE"]
    #[inline(always)]
    #[must_use]
    pub fn ttmie(&mut self) -> TTMIE_W<5> {
        TTMIE_W::new(self)
    }
    #[doc = "Bit 6 - SWEE"]
    #[inline(always)]
    #[must_use]
    pub fn swee(&mut self) -> SWEE_W<6> {
        SWEE_W::new(self)
    }
    #[doc = "Bit 7 - GTWE"]
    #[inline(always)]
    #[must_use]
    pub fn gtwe(&mut self) -> GTWE_W<7> {
        GTWE_W::new(self)
    }
    #[doc = "Bit 8 - GTDE"]
    #[inline(always)]
    #[must_use]
    pub fn gtde(&mut self) -> GTDE_W<8> {
        GTDE_W::new(self)
    }
    #[doc = "Bit 9 - GTEE"]
    #[inline(always)]
    #[must_use]
    pub fn gtee(&mut self) -> GTEE_W<9> {
        GTEE_W::new(self)
    }
    #[doc = "Bit 10 - TXUE"]
    #[inline(always)]
    #[must_use]
    pub fn txue(&mut self) -> TXUE_W<10> {
        TXUE_W::new(self)
    }
    #[doc = "Bit 11 - TXOE"]
    #[inline(always)]
    #[must_use]
    pub fn txoe(&mut self) -> TXOE_W<11> {
        TXOE_W::new(self)
    }
    #[doc = "Bit 12 - SE1E"]
    #[inline(always)]
    #[must_use]
    pub fn se1e(&mut self) -> SE1E_W<12> {
        SE1E_W::new(self)
    }
    #[doc = "Bit 13 - SE2E"]
    #[inline(always)]
    #[must_use]
    pub fn se2e(&mut self) -> SE2E_W<13> {
        SE2E_W::new(self)
    }
    #[doc = "Bit 14 - ELCE"]
    #[inline(always)]
    #[must_use]
    pub fn elce(&mut self) -> ELCE_W<14> {
        ELCE_W::new(self)
    }
    #[doc = "Bit 15 - IWTE"]
    #[inline(always)]
    #[must_use]
    pub fn iwte(&mut self) -> IWTE_W<15> {
        IWTE_W::new(self)
    }
    #[doc = "Bit 16 - WTE"]
    #[inline(always)]
    #[must_use]
    pub fn wte(&mut self) -> WTE_W<16> {
        WTE_W::new(self)
    }
    #[doc = "Bit 17 - AWE"]
    #[inline(always)]
    #[must_use]
    pub fn awe(&mut self) -> AWE_W<17> {
        AWE_W::new(self)
    }
    #[doc = "Bit 18 - CERE"]
    #[inline(always)]
    #[must_use]
    pub fn cere(&mut self) -> CERE_W<18> {
        CERE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The settings in the TT interrupt enable register determine which status changes in the TT interrupt register will result in an interrupt.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ttie](index.html) module"]
pub struct FDCAN_TTIE_SPEC;
impl crate::RegisterSpec for FDCAN_TTIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_ttie::R](R) reader structure"]
impl crate::Readable for FDCAN_TTIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_ttie::W](W) writer structure"]
impl crate::Writable for FDCAN_TTIE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TTIE to value 0"]
impl crate::Resettable for FDCAN_TTIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
