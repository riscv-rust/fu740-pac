#[doc = "Register `corepllsel` reader"]
pub struct R(crate::R<COREPLLSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COREPLLSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COREPLLSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COREPLLSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `corepllsel` writer"]
pub struct W(crate::W<COREPLLSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COREPLLSEL_SPEC>;
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
impl From<crate::W<COREPLLSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COREPLLSEL_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [corepllsel](index.html) module"]
pub struct COREPLLSEL_SPEC;
impl crate::RegisterSpec for COREPLLSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [corepllsel::R](R) reader structure"]
impl crate::Readable for COREPLLSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [corepllsel::W](W) writer structure"]
impl crate::Writable for COREPLLSEL_SPEC {
    type Writer = W;
}
