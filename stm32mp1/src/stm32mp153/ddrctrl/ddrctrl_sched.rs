#[doc = "Register `DDRCTRL_SCHED` reader"]
pub struct R(crate::R<DDRCTRL_SCHED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_SCHED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_SCHED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_SCHED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_SCHED` writer"]
pub struct W(crate::W<DDRCTRL_SCHED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_SCHED_SPEC>;
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
impl From<crate::W<DDRCTRL_SCHED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_SCHED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORCE_LOW_PRI_N` reader - FORCE_LOW_PRI_N"]
pub type FORCE_LOW_PRI_N_R = crate::BitReader<bool>;
#[doc = "Field `FORCE_LOW_PRI_N` writer - FORCE_LOW_PRI_N"]
pub type FORCE_LOW_PRI_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_SCHED_SPEC, bool, O>;
#[doc = "Field `PREFER_WRITE` reader - PREFER_WRITE"]
pub type PREFER_WRITE_R = crate::BitReader<bool>;
#[doc = "Field `PREFER_WRITE` writer - PREFER_WRITE"]
pub type PREFER_WRITE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRCTRL_SCHED_SPEC, bool, O>;
#[doc = "Field `PAGECLOSE` reader - PAGECLOSE"]
pub type PAGECLOSE_R = crate::BitReader<bool>;
#[doc = "Field `PAGECLOSE` writer - PAGECLOSE"]
pub type PAGECLOSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRCTRL_SCHED_SPEC, bool, O>;
#[doc = "Field `LPR_NUM_ENTRIES` reader - LPR_NUM_ENTRIES"]
pub type LPR_NUM_ENTRIES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPR_NUM_ENTRIES` writer - LPR_NUM_ENTRIES"]
pub type LPR_NUM_ENTRIES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_SCHED_SPEC, u8, u8, 4, O>;
#[doc = "Field `GO2CRITICAL_HYSTERESIS` reader - GO2CRITICAL_HYSTERESIS"]
pub type GO2CRITICAL_HYSTERESIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GO2CRITICAL_HYSTERESIS` writer - GO2CRITICAL_HYSTERESIS"]
pub type GO2CRITICAL_HYSTERESIS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_SCHED_SPEC, u8, u8, 8, O>;
#[doc = "Field `RDWR_IDLE_GAP` reader - RDWR_IDLE_GAP"]
pub type RDWR_IDLE_GAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDWR_IDLE_GAP` writer - RDWR_IDLE_GAP"]
pub type RDWR_IDLE_GAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_SCHED_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - FORCE_LOW_PRI_N"]
    #[inline(always)]
    pub fn force_low_pri_n(&self) -> FORCE_LOW_PRI_N_R {
        FORCE_LOW_PRI_N_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PREFER_WRITE"]
    #[inline(always)]
    pub fn prefer_write(&self) -> PREFER_WRITE_R {
        PREFER_WRITE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PAGECLOSE"]
    #[inline(always)]
    pub fn pageclose(&self) -> PAGECLOSE_R {
        PAGECLOSE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:11 - LPR_NUM_ENTRIES"]
    #[inline(always)]
    pub fn lpr_num_entries(&self) -> LPR_NUM_ENTRIES_R {
        LPR_NUM_ENTRIES_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - GO2CRITICAL_HYSTERESIS"]
    #[inline(always)]
    pub fn go2critical_hysteresis(&self) -> GO2CRITICAL_HYSTERESIS_R {
        GO2CRITICAL_HYSTERESIS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - RDWR_IDLE_GAP"]
    #[inline(always)]
    pub fn rdwr_idle_gap(&self) -> RDWR_IDLE_GAP_R {
        RDWR_IDLE_GAP_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - FORCE_LOW_PRI_N"]
    #[inline(always)]
    #[must_use]
    pub fn force_low_pri_n(&mut self) -> FORCE_LOW_PRI_N_W<0> {
        FORCE_LOW_PRI_N_W::new(self)
    }
    #[doc = "Bit 1 - PREFER_WRITE"]
    #[inline(always)]
    #[must_use]
    pub fn prefer_write(&mut self) -> PREFER_WRITE_W<1> {
        PREFER_WRITE_W::new(self)
    }
    #[doc = "Bit 2 - PAGECLOSE"]
    #[inline(always)]
    #[must_use]
    pub fn pageclose(&mut self) -> PAGECLOSE_W<2> {
        PAGECLOSE_W::new(self)
    }
    #[doc = "Bits 8:11 - LPR_NUM_ENTRIES"]
    #[inline(always)]
    #[must_use]
    pub fn lpr_num_entries(&mut self) -> LPR_NUM_ENTRIES_W<8> {
        LPR_NUM_ENTRIES_W::new(self)
    }
    #[doc = "Bits 16:23 - GO2CRITICAL_HYSTERESIS"]
    #[inline(always)]
    #[must_use]
    pub fn go2critical_hysteresis(&mut self) -> GO2CRITICAL_HYSTERESIS_W<16> {
        GO2CRITICAL_HYSTERESIS_W::new(self)
    }
    #[doc = "Bits 24:30 - RDWR_IDLE_GAP"]
    #[inline(always)]
    #[must_use]
    pub fn rdwr_idle_gap(&mut self) -> RDWR_IDLE_GAP_W<24> {
        RDWR_IDLE_GAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL scheduler control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_sched](index.html) module"]
pub struct DDRCTRL_SCHED_SPEC;
impl crate::RegisterSpec for DDRCTRL_SCHED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_sched::R](R) reader structure"]
impl crate::Readable for DDRCTRL_SCHED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_sched::W](W) writer structure"]
impl crate::Writable for DDRCTRL_SCHED_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_SCHED to value 0x0805"]
impl crate::Resettable for DDRCTRL_SCHED_SPEC {
    const RESET_VALUE: Self::Ux = 0x0805;
}
