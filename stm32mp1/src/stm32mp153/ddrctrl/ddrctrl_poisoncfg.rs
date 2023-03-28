#[doc = "Register `DDRCTRL_POISONCFG` reader"]
pub struct R(crate::R<DDRCTRL_POISONCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_POISONCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_POISONCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_POISONCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_POISONCFG` writer"]
pub struct W(crate::W<DDRCTRL_POISONCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_POISONCFG_SPEC>;
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
impl From<crate::W<DDRCTRL_POISONCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_POISONCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WR_POISON_SLVERR_EN` reader - WR_POISON_SLVERR_EN"]
pub type WR_POISON_SLVERR_EN_R = crate::BitReader<bool>;
#[doc = "Field `WR_POISON_SLVERR_EN` writer - WR_POISON_SLVERR_EN"]
pub type WR_POISON_SLVERR_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_POISONCFG_SPEC, bool, O>;
#[doc = "Field `WR_POISON_INTR_EN` reader - WR_POISON_INTR_EN"]
pub type WR_POISON_INTR_EN_R = crate::BitReader<bool>;
#[doc = "Field `WR_POISON_INTR_EN` writer - WR_POISON_INTR_EN"]
pub type WR_POISON_INTR_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_POISONCFG_SPEC, bool, O>;
#[doc = "Field `WR_POISON_INTR_CLR` reader - WR_POISON_INTR_CLR"]
pub type WR_POISON_INTR_CLR_R = crate::BitReader<bool>;
#[doc = "Field `WR_POISON_INTR_CLR` writer - WR_POISON_INTR_CLR"]
pub type WR_POISON_INTR_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_POISONCFG_SPEC, bool, O>;
#[doc = "Field `RD_POISON_SLVERR_EN` reader - RD_POISON_SLVERR_EN"]
pub type RD_POISON_SLVERR_EN_R = crate::BitReader<bool>;
#[doc = "Field `RD_POISON_SLVERR_EN` writer - RD_POISON_SLVERR_EN"]
pub type RD_POISON_SLVERR_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_POISONCFG_SPEC, bool, O>;
#[doc = "Field `RD_POISON_INTR_EN` reader - RD_POISON_INTR_EN"]
pub type RD_POISON_INTR_EN_R = crate::BitReader<bool>;
#[doc = "Field `RD_POISON_INTR_EN` writer - RD_POISON_INTR_EN"]
pub type RD_POISON_INTR_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_POISONCFG_SPEC, bool, O>;
#[doc = "Field `RD_POISON_INTR_CLR` reader - RD_POISON_INTR_CLR"]
pub type RD_POISON_INTR_CLR_R = crate::BitReader<bool>;
#[doc = "Field `RD_POISON_INTR_CLR` writer - RD_POISON_INTR_CLR"]
pub type RD_POISON_INTR_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_POISONCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - WR_POISON_SLVERR_EN"]
    #[inline(always)]
    pub fn wr_poison_slverr_en(&self) -> WR_POISON_SLVERR_EN_R {
        WR_POISON_SLVERR_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - WR_POISON_INTR_EN"]
    #[inline(always)]
    pub fn wr_poison_intr_en(&self) -> WR_POISON_INTR_EN_R {
        WR_POISON_INTR_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - WR_POISON_INTR_CLR"]
    #[inline(always)]
    pub fn wr_poison_intr_clr(&self) -> WR_POISON_INTR_CLR_R {
        WR_POISON_INTR_CLR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - RD_POISON_SLVERR_EN"]
    #[inline(always)]
    pub fn rd_poison_slverr_en(&self) -> RD_POISON_SLVERR_EN_R {
        RD_POISON_SLVERR_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - RD_POISON_INTR_EN"]
    #[inline(always)]
    pub fn rd_poison_intr_en(&self) -> RD_POISON_INTR_EN_R {
        RD_POISON_INTR_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - RD_POISON_INTR_CLR"]
    #[inline(always)]
    pub fn rd_poison_intr_clr(&self) -> RD_POISON_INTR_CLR_R {
        RD_POISON_INTR_CLR_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WR_POISON_SLVERR_EN"]
    #[inline(always)]
    #[must_use]
    pub fn wr_poison_slverr_en(&mut self) -> WR_POISON_SLVERR_EN_W<0> {
        WR_POISON_SLVERR_EN_W::new(self)
    }
    #[doc = "Bit 4 - WR_POISON_INTR_EN"]
    #[inline(always)]
    #[must_use]
    pub fn wr_poison_intr_en(&mut self) -> WR_POISON_INTR_EN_W<4> {
        WR_POISON_INTR_EN_W::new(self)
    }
    #[doc = "Bit 8 - WR_POISON_INTR_CLR"]
    #[inline(always)]
    #[must_use]
    pub fn wr_poison_intr_clr(&mut self) -> WR_POISON_INTR_CLR_W<8> {
        WR_POISON_INTR_CLR_W::new(self)
    }
    #[doc = "Bit 16 - RD_POISON_SLVERR_EN"]
    #[inline(always)]
    #[must_use]
    pub fn rd_poison_slverr_en(&mut self) -> RD_POISON_SLVERR_EN_W<16> {
        RD_POISON_SLVERR_EN_W::new(self)
    }
    #[doc = "Bit 20 - RD_POISON_INTR_EN"]
    #[inline(always)]
    #[must_use]
    pub fn rd_poison_intr_en(&mut self) -> RD_POISON_INTR_EN_W<20> {
        RD_POISON_INTR_EN_W::new(self)
    }
    #[doc = "Bit 24 - RD_POISON_INTR_CLR"]
    #[inline(always)]
    #[must_use]
    pub fn rd_poison_intr_clr(&mut self) -> RD_POISON_INTR_CLR_W<24> {
        RD_POISON_INTR_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AXI Poison configuration register common for all AXI ports.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_poisoncfg](index.html) module"]
pub struct DDRCTRL_POISONCFG_SPEC;
impl crate::RegisterSpec for DDRCTRL_POISONCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_poisoncfg::R](R) reader structure"]
impl crate::Readable for DDRCTRL_POISONCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_poisoncfg::W](W) writer structure"]
impl crate::Writable for DDRCTRL_POISONCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_POISONCFG to value 0x0011_0011"]
impl crate::Resettable for DDRCTRL_POISONCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0011_0011;
}
